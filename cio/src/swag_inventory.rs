use async_trait::async_trait;
use barcoders::generators::image::*;
use barcoders::generators::svg::*;
use barcoders::sym::code39::*;
use google_drive::GoogleDrive;
use image::{DynamicImage, ImageFormat};
use lopdf::content::{Content, Operation};
use lopdf::{Document, Object, Stream, StringFormat};
use macros::db;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::airtable::{AIRTABLE_BASE_ID_SWAG, AIRTABLE_SWAG_INVENTORY_ITEMS_TABLE};
use crate::core::UpdateAirtableRecord;
use crate::db::Database;
use crate::schema::swag_inventory_items;
use crate::utils::get_gsuite_token;

#[db {
    new_struct_name = "SwagInventoryItem",
    airtable_base_id = "AIRTABLE_BASE_ID_SWAG",
    airtable_table = "AIRTABLE_SWAG_INVENTORY_ITEMS_TABLE",
    match_on = {
        "item" = "String",
        "size" = "String",
    },
}]
#[derive(Debug, Insertable, AsChangeset, PartialEq, Clone, JsonSchema, Deserialize, Serialize)]
#[table_name = "swag_inventory_items"]
pub struct NewSwagInventoryItem {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub size: String,
    #[serde(default)]
    pub current_stock: i32,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub item: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        serialize_with = "airtable_api::barcode_format_as_string::serialize",
        deserialize_with = "airtable_api::barcode_format_as_string::deserialize"
    )]
    pub barcode: String,

    #[serde(default, skip_serializing_if = "String::is_empty", deserialize_with = "airtable_api::attachment_format_as_string::deserialize")]
    pub barcode_png: String,
    #[serde(default, skip_serializing_if = "String::is_empty", deserialize_with = "airtable_api::attachment_format_as_string::deserialize")]
    pub barcode_svg: String,
    #[serde(default, skip_serializing_if = "String::is_empty", deserialize_with = "airtable_api::attachment_format_as_string::deserialize")]
    pub barcode_pdf_label: String,

    /// This is populated by Airtable.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub link_to_item: Vec<String>,
}

/// Implement updating the Airtable record for a SwagInventoryItem.
#[async_trait]
impl UpdateAirtableRecord<SwagInventoryItem> for SwagInventoryItem {
    async fn update_airtable_record(&mut self, record: SwagInventoryItem) {
        if !record.link_to_item.is_empty() {
            self.link_to_item = record.link_to_item;
        }

        // This is a funtion in Airtable so we can't update it.
        self.name = "".to_string();
    }
}

impl NewSwagInventoryItem {
    pub fn generate_barcode(&mut self) {
        self.barcode = self
            .name
            .to_uppercase()
            .replace("FIRST EDITION", "1ED")
            .replace("SECOND EDITION", "2ED")
            .replace("THIRD EDITION", "3ED")
            // TODO: Find another way to do this so that it doesn't break eventually.
            .replace("FOURTH EDITION", "4ED")
            .replace(' ', "")
            .replace('/', "")
            .replace('(', "")
            .replace(')', "")
            .replace('-', "")
            .replace("'", "")
            .trim()
            .to_string();
    }

    pub async fn generate_barcode_images(&mut self, drive_client: &GoogleDrive) {
        // Generate the barcode.
        // "Name" is automatically generated by Airtable from the item and the size.
        if !self.name.is_empty() {
            let bucket = "oxide_automated_documents";
            // Generate the barcode svg and png.
            let barcode = Code39::new(&self.barcode).unwrap();
            let png = Image::png(50); // You must specify the height in pixels.
            let encoded = barcode.encode();

            // Image generators return a Result<Vec<u8>, barcoders::error::Error) of encoded bytes.
            let png_bytes = png.generate(&encoded[..]).unwrap();
            let mut file_name = format!("{}.png", self.name);

            // Create or update the files in the google_drive.
            let png_file = drive_client.upload_to_cloud_storage(bucket, &file_name, "image/png", &png_bytes, true).await.unwrap();
            self.barcode_png = png_file.media_link.to_string();

            // Now do the SVG.
            let svg = SVG::new(200); // You must specify the height in pixels.
            let svg_data: String = svg.generate(&encoded).unwrap();
            let svg_bytes = svg_data.as_bytes();

            file_name = format!("{}.svg", self.name);

            // Create or update the files in the google_drive.
            let svg_file = drive_client.upload_to_cloud_storage(bucket, &file_name, "image/svg+xml", &svg_bytes, true).await.unwrap();
            self.barcode_svg = svg_file.media_link.to_string();

            // Generate the barcode label.
            let label_bytes = self.generate_pdf_barcode_label(&png_bytes);
            file_name = format!("{} - Barcode Label.pdf", self.name);
            // Create or update the files in the google_drive.
            let label_file = drive_client.upload_to_cloud_storage(bucket, &file_name, "application/pdf", &label_bytes, true).await.unwrap();
            self.barcode_pdf_label = label_file.media_link.to_string();
        }
    }

    // Get the bytes for a pdf barcode label.
    pub fn generate_pdf_barcode_label(&self, png_bytes: &[u8]) -> Vec<u8> {
        let pdf_width = 288.0;
        let pdf_height = 432.0;
        let mut doc = Document::with_version("1.5");
        let pages_id = doc.new_object_id();
        let font_id = doc.add_object(dictionary! {
            "Type" => "Font",
            "Subtype" => "Type1",
            "BaseFont" => "Courier",
        });
        let resources_id = doc.add_object(dictionary! {
            "Font" => dictionary! {
                "F1" => font_id,
            },
        });
        let content = Content {
            operations: vec![
                Operation::new("BT", vec![]),
                Operation::new("Tf", vec!["F1".into(), 8.into()]),
                Operation::new("Td", vec![10.into(), 10.into()]),
                Operation::new("Tj", vec![Object::string_literal(self.name.to_string())]),
                Operation::new("ET", vec![]),
            ],
        };
        let content_id = doc.add_object(Stream::new(dictionary! {}, content.encode().unwrap()));
        let page_id = doc.add_object(dictionary! {
            "Type" => "Page",
            "Parent" => pages_id,
            "Contents" => content_id,
        });

        let pages = dictionary! {
            "Type" => "Pages",
            "Kids" => vec![page_id.into()],
            "Count" => 1,
            "Resources" => resources_id,
            // This should be (4 in x 6 in) for the rollo printer.
            // You get `pts` by (inches * 72).
            "MediaBox" => vec![0.into(), 0.into(),pdf_width.into(), pdf_height.into()],
        };
        doc.objects.insert(pages_id, Object::Dictionary(pages));
        let catalog_id = doc.add_object(dictionary! {
            "Type" => "Catalog",
            "Pages" => pages_id,
        });
        doc.trailer.set("Root", catalog_id);

        let logo_bytes = include_bytes!("oxide_logo.png");
        let (mut doc, logo_stream, logo_info) = image_to_pdf_object(doc, logo_bytes);
        // Center the logo at the top of the pdf.
        doc.insert_image(
            page_id,
            logo_stream,
            ((pdf_width - logo_info.width as f64) / 2.0, pdf_height - logo_info.height as f64 - 10.0),
            (logo_info.width.into(), logo_info.height.into()),
        )
        .unwrap();

        let (mut doc, img_stream, info) = image_to_pdf_object(doc, png_bytes);
        // Center the barcode at the top of the pdf.
        doc.insert_image(
            page_id,
            img_stream,
            ((pdf_width - info.width as f64) / 2.0, pdf_height - info.height as f64 - logo_info.height as f64 - 20.0),
            (info.width.into(), info.height.into()),
        )
        .unwrap();
        doc.compress();

        // Save the PDF
        let mut buffer = Vec::new();
        doc.save_to(&mut buffer).unwrap();
        buffer
    }

    pub async fn expand(&mut self, drive_client: &GoogleDrive) {
        self.generate_barcode();
        self.generate_barcode_images(drive_client).await;
    }
}

pub fn image_to_pdf_object(mut doc: Document, png_bytes: &[u8]) -> (Document, Stream, crate::png::PngInfo) {
    // Insert our barcode image.
    let info = crate::png::get_info(png_bytes);

    let bytes = if info.interlace || info.color_type >= 4 {
        let img = image::load_from_memory(png_bytes).unwrap();
        let mut result = Vec::new();

        match info.color_type {
            4 => match info.depth {
                8 => DynamicImage::ImageLuma8(img.into_luma8()),
                16 => DynamicImage::ImageLuma16(img.into_luma16()),
                _ => panic!(""),
            },
            6 => match info.depth {
                8 => DynamicImage::ImageRgb8(img.into_rgb8()),
                16 => DynamicImage::ImageRgb16(img.into_rgb16()),
                _ => panic!(""),
            },
            _ => img,
        }
        .write_to(&mut result, ImageFormat::Png)
        .unwrap();
        result
    } else {
        png_bytes.into()
    };

    let colors = if let 0 | 3 | 4 = info.color_type { 1 } else { 3 };

    let idat = crate::png::get_idat(&bytes[..]);

    let cs: Object = match info.color_type {
        0 | 2 | 4 | 6 => {
            if let Some(ref raw) = info.icc {
                let icc_id = doc.add_object(Stream::new(
                    dictionary! {
                        "N" => colors,
                        "Alternate" => if let 0 | 4 = info.color_type { "DeviceGray" } else { "DeviceRGB" },
                        "Length" => raw.len() as u32,
                        "Filter" => "FlateDecode"
                    },
                    raw.to_vec(),
                ));
                vec!["ICCBased".into(), icc_id.into()].into()
            } else {
                if let 0 | 4 = info.color_type { "DeviceGray" } else { "DeviceRGB" }.into()
            }
        }

        3 => {
            let palette = info.clone().palette.unwrap();
            vec!["Indexed".into(), "DeviceRGB".into(), (palette.1 - 1).into(), Object::String(palette.0, StringFormat::Hexadecimal)].into()
        }

        _ => panic!("unexpected color type found: {}", info.color_type),
    };

    let img_stream = Stream::new(
        dictionary! {
            "Type" => "XObject",
            "Subtype" => "Image",
            "Filter" => "FlateDecode",
            "BitsPerComponent" => info.depth,
            "Length" => idat.len() as u32,
            "Width" => info.width,
            "Height" => info.height,
            "DecodeParms" => dictionary!{
                "BitsPerComponent" => info.depth,
                "Predictor" => 15,
                "Columns" => info.width,
                "Colors" => colors
            },
            "ColorSpace" => cs,
        },
        idat,
    );

    (doc, img_stream, info)
}

/// Sync software vendors from Airtable.
pub async fn refresh_swag_inventory_items() {
    let db = Database::new();

    // Get gsuite token.
    let token = get_gsuite_token("").await;

    // Initialize the Google Drive client.
    let drive_client = GoogleDrive::new(token);

    // Get all the records from Airtable.
    let results: Vec<airtable_api::Record<SwagInventoryItem>> = SwagInventoryItem::airtable().list_records(&SwagInventoryItem::airtable_table(), "Grid view", vec![]).await.unwrap();
    for inventory_item_record in results {
        let mut inventory_item: NewSwagInventoryItem = inventory_item_record.fields.into();
        inventory_item.expand(&drive_client).await;

        let mut db_inventory_item = inventory_item.upsert_in_db(&db);
        db_inventory_item.airtable_record_id = inventory_item_record.id.to_string();
        db_inventory_item.update(&db).await;
    }
}

#[cfg(test)]
mod tests {
    use crate::swag_inventory::refresh_swag_inventory_items;

    #[ignore]
    #[tokio::test(flavor = "multi_thread")]
    async fn test_cron_swag_inventory_items() {
        refresh_swag_inventory_items().await;
    }
}
