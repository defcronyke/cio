use std::env;

use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::code_that_should_be_generated::{
    Applicant, Building, ConferenceRoom, GithubLabel, Group, Link, User,
};
use crate::configs::{
    BuildingConfig, GroupConfig, LabelConfig, LinkConfig, ResourceConfig,
    UserConfig,
};
use crate::models::NewApplicant;
use crate::schema::{
    applicants, buildings, conference_rooms, github_labels, groups, links,
    users,
};

pub struct Database {
    conn: PgConnection,
}

impl Default for Database {
    fn default() -> Self {
        let database_url =
            env::var("CIO_DATABASE_URL").expect("CIO_DATABASE_URL must be set");

        Database {
            conn: PgConnection::establish(&database_url).unwrap_or_else(|e| {
                panic!("error connecting to {}: {}", database_url, e)
            }),
        }
    }
}

// TODO: more gracefully handle errors
// TODO: possibly generate all this boilerplate as well.
// TODO: generate all the diesal duplicates as well.
impl Database {
    /// Establish a connection to the database.
    pub fn new() -> Database {
        Default::default()
    }

    pub fn upsert_applicant(&self, applicant: &NewApplicant) -> Applicant {
        // See if we already have the applicant in the database.
        match applicants::dsl::applicants
            .filter(
                applicants::dsl::email.eq(applicant.email.to_string()),
                applicants::dsl::sheet_id.eq(applicant.sheet_id.to_string()),
            )
            .limit(1)
            .load::<Applicant>(&self.conn)
        {
            Ok(r) => {
                if r.is_empty() {
                    // We don't have the applicant in the database so we need to add it.
                    // That will happen below.
                } else {
                    let a = r.get(0).unwrap();

                    // Update the applicant.
                    return diesel::update(a)
                        .set(applicant)
                        .get_result::<Applicant>(&self.conn)
                        .unwrap_or_else(|e| {
                            panic!("unable to update applicant {}: {}", a.id, e)
                        });
                }
            }
            Err(e) => {
                println!("[db] on err: {:?}; we don't have the applicant in the database, adding it", e);
            }
        }

        diesel::insert_into(applicants::table)
            .values(applicant)
            .get_result(&self.conn)
            .unwrap_or_else(|e| panic!("creating applicant failed: {}", e))
    }

    pub fn upsert_building(&self, building: &BuildingConfig) -> Building {
        // See if we already have the building in the database.
        match buildings::dsl::buildings
            .filter(buildings::dsl::name.eq(building.name.to_string()))
            .limit(1)
            .load::<Building>(&self.conn)
        {
            Ok(r) => {
                if r.is_empty() {
                    // We don't have the building in the database so we need to add it.
                    // That will happen below.
                } else {
                    let b = r.get(0).unwrap();

                    // Update the building.
                    return diesel::update(b)
                        .set(building)
                        .get_result::<Building>(&self.conn)
                        .unwrap_or_else(|e| {
                            panic!("unable to update building {}: {}", b.id, e)
                        });
                }
            }
            Err(e) => {
                println!("[db] on err: {:?}; we don't have the building in the database, adding it", e);
            }
        }

        diesel::insert_into(buildings::table)
            .values(building)
            .get_result(&self.conn)
            .unwrap_or_else(|e| panic!("creating building failed: {}", e))
    }

    pub fn upsert_conference_room(
        &self,
        conference_room: &ResourceConfig,
    ) -> ConferenceRoom {
        // See if we already have the conference_room in the database.
        match conference_rooms::dsl::conference_rooms
            .filter(
                conference_rooms::dsl::name
                    .eq(conference_room.name.to_string()),
            )
            .limit(1)
            .load::<ConferenceRoom>(&self.conn)
        {
            Ok(r) => {
                if r.is_empty() {
                    // We don't have the conference_room in the database so we need to add it.
                    // That will happen below.
                } else {
                    let c = r.get(0).unwrap();

                    // Update the conference_room.
                    return diesel::update(c)
                        .set(conference_room)
                        .get_result::<ConferenceRoom>(&self.conn)
                        .unwrap_or_else(|e| {
                            panic!(
                                "unable to update conference_room {}: {}",
                                c.id, e
                            )
                        });
                }
            }
            Err(e) => {
                println!("[db] on err: {:?}; we don't have the conference_room in the database, adding it", e);
            }
        }

        diesel::insert_into(conference_rooms::table)
            .values(conference_room)
            .get_result(&self.conn)
            .unwrap_or_else(|e| {
                panic!("creating conference_room failed: {}", e)
            })
    }

    pub fn upsert_github_label(
        &self,
        github_label: &LabelConfig,
    ) -> GithubLabel {
        // See if we already have the github_label in the database.
        match github_labels::dsl::github_labels
            .filter(github_labels::dsl::name.eq(github_label.name.to_string()))
            .limit(1)
            .load::<GithubLabel>(&self.conn)
        {
            Ok(r) => {
                if r.is_empty() {
                    // We don't have the github_label in the database so we need to add it.
                    // That will happen below.
                } else {
                    let label = r.get(0).unwrap();

                    // Update the github_label.
                    return diesel::update(label)
                        .set(github_label)
                        .get_result::<GithubLabel>(&self.conn)
                        .unwrap_or_else(|e| {
                            panic!(
                                "unable to update github_label {}: {}",
                                label.id, e
                            )
                        });
                }
            }
            Err(e) => {
                println!("[db] on err: {:?}; we don't have the github_label in the database, adding it", e);
            }
        }

        diesel::insert_into(github_labels::table)
            .values(github_label)
            .get_result(&self.conn)
            .unwrap_or_else(|e| panic!("creating github_label failed: {}", e))
    }

    pub fn upsert_group(&self, group: &GroupConfig) -> Group {
        // See if we already have the group in the database.
        match groups::dsl::groups
            .filter(groups::dsl::name.eq(group.name.to_string()))
            .limit(1)
            .load::<Group>(&self.conn)
        {
            Ok(r) => {
                if r.is_empty() {
                    // We don't have the group in the database so we need to add it.
                    // That will happen below.
                } else {
                    let g = r.get(0).unwrap();

                    // Update the group.
                    return diesel::update(g)
                        .set(group)
                        .get_result::<Group>(&self.conn)
                        .unwrap_or_else(|e| {
                            panic!("unable to update group {}: {}", g.id, e)
                        });
                }
            }
            Err(e) => {
                println!("[db] on err: {:?}; we don't have the group in the database, adding it", e);
            }
        }

        diesel::insert_into(groups::table)
            .values(group)
            .get_result(&self.conn)
            .unwrap_or_else(|e| panic!("creating group failed: {}", e))
    }

    pub fn upsert_link(&self, link: &LinkConfig) -> Link {
        // See if we already have the link in the database.
        match links::dsl::links
            .filter(links::dsl::name.eq(link.name.to_string()))
            .limit(1)
            .load::<Link>(&self.conn)
        {
            Ok(r) => {
                if r.is_empty() {
                    // We don't have the link in the database so we need to add it.
                    // That will happen below.
                } else {
                    let l = r.get(0).unwrap();

                    // Update the link.
                    return diesel::update(l)
                        .set(link)
                        .get_result::<Link>(&self.conn)
                        .unwrap_or_else(|e| {
                            panic!("unable to update link {}: {}", l.id, e)
                        });
                }
            }
            Err(e) => {
                println!("[db] on err: {:?}; we don't have the link in the database, adding it", e);
            }
        }

        diesel::insert_into(links::table)
            .values(link)
            .get_result(&self.conn)
            .unwrap_or_else(|e| panic!("creating link failed: {}", e))
    }

    pub fn upsert_user(&self, user: &UserConfig) -> User {
        // See if we already have the user in the database.
        match users::dsl::users
            .filter(users::dsl::username.eq(user.username.to_string()))
            .limit(1)
            .load::<User>(&self.conn)
        {
            Ok(r) => {
                if r.is_empty() {
                    // We don't have the user in the database so we need to add it.
                    // That will happen below.
                } else {
                    let u = r.get(0).unwrap();

                    // Update the user.
                    return diesel::update(u)
                        .set(user)
                        .get_result::<User>(&self.conn)
                        .unwrap_or_else(|e| {
                            panic!("unable to update user {}: {}", u.id, e)
                        });
                }
            }
            Err(e) => {
                println!("[db] on err: {:?}; we don't have the user in the database, adding it", e);
            }
        }

        diesel::insert_into(users::table)
            .values(user)
            .get_result(&self.conn)
            .unwrap_or_else(|e| panic!("creating user failed: {}", e))
    }
}
