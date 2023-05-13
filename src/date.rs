use chrono::{Duration, NaiveDate, Utc};

pub fn get_all_dates() -> Vec<NaiveDate> {
    get_dates_between(first(), today())
}

/// Today's date, as `NaiveDate`
fn today() -> NaiveDate {
    Utc::now().date_naive()
}

/// Date of first comic, as `NaiveDate`
fn first() -> NaiveDate {
    NaiveDate::from_ymd_opt(1978, 6, 19).expect("Static date failed to parse")
}

fn get_dates_between(start: NaiveDate, end: NaiveDate) -> Vec<NaiveDate> {
    let mut dates = Vec::new();

    let mut current = start;
    while current <= end {
        dates.push(current);
        current += Duration::days(1);
    }

    dates
}
