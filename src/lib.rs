pub mod date;
mod garf;
mod img;

use std::fs::DirEntry;

use chrono::{Datelike, Duration, NaiveDate};
use reqwest::Client;

pub fn get_dates_between(start: NaiveDate, end: NaiveDate) -> Vec<NaiveDate> {
    let mut dates = Vec::new();

    let mut current = start;
    while current <= end {
        dates.push(current);
        current += Duration::days(1);
    }

    dates
}

pub fn date_from_filename(filename: &str) -> Option<NaiveDate> {
    let name = filename.split('/').last()?.split('.').next()?;
    let mut parts = name.split('-');

    let year = parts.next()?;
    let month = parts.next()?;
    let day = parts.next()?;

    let year: i32 = year.parse().ok()?;
    let month: u32 = month.parse().ok()?;
    let day: u32 = day.parse().ok()?;

    NaiveDate::from_ymd_opt(year, month, day)
}

/// Convert `NaiveDate` to YYYY/MM/DD format
fn date_to_string(date: NaiveDate, separator: &str) -> String {
    date.year().to_string()
        + separator
        + &date.month().to_string()
        + separator
        + &date.day().to_string()
}

pub fn filename_from_dir_entry(dir_entry: DirEntry) -> Option<String> {
    let name = dir_entry.file_name();
    Some(name.to_str()?.to_string())
}

pub async fn fetch_and_save_comic(
    client: &Client,
    date: NaiveDate,
    folder: &str,
) -> Result<(), ()> {
    println!("    {date}  1. Fetching url");

    let url = garf::comic_url(client, date).await.unwrap();

    println!("    {date}  2. Fetching image from {url}");

    let filepath = format!("{}/{}.png", folder, date_to_string(date, "-"));
    img::save_image(client, &url, &filepath).await.unwrap();

    println!("    {date}  3. Saved to {filepath}");

    Ok(())
}
