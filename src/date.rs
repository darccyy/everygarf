use chrono::{Duration, NaiveDate, Utc};

/// Returns list of all dates, from date of first comic to yesterday
pub fn get_all_dates() -> Vec<NaiveDate> {
    get_dates_between(first(), yesterday())
}

/// Yesterday's date, as `NaiveDate`
fn yesterday() -> NaiveDate {
    Utc::now().date_naive() - Duration::days(1)
}

/// Date of first comic, as `NaiveDate`
fn first() -> NaiveDate {
    NaiveDate::from_ymd_opt(1978, 6, 19).expect("Static date failed to parse")
}

/// Returns list of all dates, between two `NaiveDate`s
fn get_dates_between(start: NaiveDate, end: NaiveDate) -> Vec<NaiveDate> {
    let mut dates = Vec::new();

    let mut current = start;
    while current <= end {
        dates.push(current);
        current += Duration::days(1);
    }

    dates
}
