use chrono::{NaiveDate, Utc};

/// Today's date, as `NaiveDate`
pub fn today() -> NaiveDate {
    Utc::now().date_naive()
}

/// Date of first comic, as `NaiveDate`
pub fn first() -> NaiveDate {
    NaiveDate::from_ymd_opt(1978, 6, 19).expect("Static date failed to parse")
}
