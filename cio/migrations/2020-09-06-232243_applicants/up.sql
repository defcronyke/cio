CREATE TABLE applicants (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    role VARCHAR NOT NULL,
    sheet_id VARCHAR NOT NULL,
    status VARCHAR NOT NULL,
    raw_status VARCHAR NOT NULL,
    submitted_time TIMESTAMPTZ NOT NULL,
    email VARCHAR NOT NULL,
    phone VARCHAR NOT NULL,
    country_code VARCHAR NOT NULL,
    location VARCHAR NOT NULL,
    latitude REAL NOT NULL DEFAULT 0,
    longitude REAL NOT NULL DEFAULT 0,
    github VARCHAR NOT NULL,
    gitlab VARCHAR NOT NULL,
    linkedin VARCHAR NOT NULL,
    portfolio VARCHAR NOT NULL,
    website VARCHAR NOT NULL,
    resume VARCHAR NOT NULL,
    materials VARCHAR NOT NULL,
    sent_email_received BOOLEAN NOT NULL DEFAULT 'f',
    sent_email_follow_up BOOLEAN NOT NULL DEFAULT 'f',
    rejection_sent_date_time TIMESTAMPTZ DEFAULT NULL,
    value_reflected VARCHAR NOT NULL,
    value_violated VARCHAR NOT NULL,
    values_in_tension TEXT [] NOT NULL,
    resume_contents TEXT NOT NULL,
    materials_contents TEXT NOT NULL,
    work_samples TEXT NOT NULL,
    writing_samples TEXT NOT NULL,
    analysis_samples TEXT NOT NULL,
    presentation_samples TEXT NOT NULL,
    exploratory_samples TEXT NOT NULL,
    question_technically_challenging TEXT NOT NULL,
    question_proud_of TEXT NOT NULL,
    question_happiest TEXT NOT NULL,
    question_unhappiest TEXT NOT NULL,
    question_value_reflected TEXT NOT NULL,
    question_value_violated TEXT NOT NULL,
    question_values_in_tension TEXT NOT NULL,
    question_why_oxide TEXT NOT NULL,
    interview_packet VARCHAR NOT NULL,
	interviews TEXT [] NOT NULL,
    interviews_started TIMESTAMPTZ DEFAULT NULL,
    interviews_completed TIMESTAMPTZ DEFAULT NULL,
	scorers TEXT [] NOT NULL,
	scorers_completed TEXT [] NOT NULL,
    scoring_form_id VARCHAR NOT NULL,
    scoring_form_url VARCHAR NOT NULL,
    scoring_form_responses_url VARCHAR NOT NULL,
    scoring_evaluations_count INTEGER DEFAULT 0 NOT NULL,
    scoring_enthusiastic_yes_count INTEGER DEFAULT 0 NOT NULL,
    scoring_yes_count INTEGER DEFAULT 0 NOT NULL,
    scoring_pass_count INTEGER DEFAULT 0 NOT NULL,
    scoring_no_count INTEGER DEFAULT 0 NOT NULL,
    scoring_not_applicable_count INTEGER DEFAULT 0 NOT NULL,
    scoring_insufficient_experience_count INTEGER DEFAULT 0 NOT NULL,
    scoring_inapplicable_experience_count INTEGER DEFAULT 0 NOT NULL,
    scoring_job_function_yet_needed_count INTEGER DEFAULT 0 NOT NULL,
    scoring_underwhelming_materials_count INTEGER DEFAULT 0 NOT NULL,
    request_background_check BOOLEAN NOT NULL DEFAULT 'f',
    criminal_background_check_status VARCHAR NOT NULL,
    motor_vehicle_background_check_status VARCHAR NOT NULL,
    start_date DATE DEFAULT NULL,
    interested_in TEXT [] NOT NULL,
    geocode_cache VARCHAR NOT NULL,
    docusign_envelope_id VARCHAR NOT NULL,
    docusign_envelope_status VARCHAR NOT NULL,
    offer_created TIMESTAMPTZ DEFAULT NULL,
    offer_completed TIMESTAMPTZ DEFAULT NULL,
    airtable_record_id VARCHAR NOT NULL
)
