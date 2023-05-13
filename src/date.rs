use chrono::{Duration, NaiveDate, NaiveTime, Utc};

/// Returns list of all dates, from date of first comic to yesterday
pub fn get_all_dates() -> Vec<NaiveDate> {
    get_dates_between(first(), last())
}

/// Date of oldest comic, as `NaiveDate`
fn first() -> NaiveDate {
    NaiveDate::from_ymd_opt(1978, 6, 19).expect("Static date failed to parse")
}

/// Date of newest comic, as `NaiveDate`
fn last() -> NaiveDate {
    // Get naive time (UTC) for when comic is published to gocomics.com
    // Estimated time is 0000-0300 EST or 0500-08000 UTC
    // One hour margin of error is added
    let time_of_publish = NaiveTime::from_hms_opt(9, 0, 0).expect("Static time failed to parse");

    // Get current date and time
    let now = Utc::now();

    // Today if currently AFTER time of publish for todays comic
    // Yesterday if currently BEFORE time of publish for todays comic
    now.date_naive() - Duration::days(if now.time() > time_of_publish { 0 } else { 1 })
}

/// Returns list of all dates, between two `NaiveDate`s
fn get_dates_between(start: NaiveDate, end: NaiveDate) -> Vec<NaiveDate> {
    (0..=(end - start).num_days())
        .map(|days| start + Duration::days(days))
        .collect()
}
