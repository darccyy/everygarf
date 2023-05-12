pub mod date;
mod garf;
mod img;

use std::fs::DirEntry;

use chrono::{Datelike, Duration, NaiveDate};
use reqwest::Client;

pub fn get_parent_folder() -> Option<String> {
    if let Some(dir) = dirs_next::picture_dir() {
        if let Some(dir) = dir.to_str() {
            return Some(dir.to_string());
        }
    }
    None
}

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
fn date_to_string(date: NaiveDate, separator: &str, leading_zeros: bool) -> String {
    let month = date.month();
    let day = date.day();

    date.year().to_string()
        + separator
        + if leading_zeros && month < 10 { "0" } else { "" }
        + &month.to_string()
        + separator
        + if leading_zeros && day < 10 { "0" } else { "" }
        + &day.to_string()
}

pub fn filename_from_dir_entry(dir_entry: DirEntry) -> Option<String> {
    let name = dir_entry.file_name();
    Some(name.to_str()?.to_string())
}

pub async fn fetch_and_save_comic(
    client: &Client,
    date: NaiveDate,
    folder: &str,
    progress: f32,
) -> Result<(), ()> {
    const ATTEMPTS: u32 = 3;

    for i in 0..ATTEMPTS {
        match attempt_fetch_and_save(client, date, folder, progress).await {
            Ok(()) => return Ok(()),

            Err(error) => eprintln!("        [warning] Attempt {n} failed: {error:?}", n = i + 1),
        }
    }

    eprintln!("        [ERROR] Failed after {ATTEMPTS} attempts");
    return Err(());
}

pub async fn attempt_fetch_and_save(
    client: &Client,
    date: NaiveDate,
    folder: &str,
    progress: f32,
) -> Result<(), ()> {
    print_step(progress, date, 1, format!("Fetching url of image"));

    let url = garf::comic_url(client, date).await?;

    print_step(progress, date, 2, format!("Fetching image from {url}"));

    let filepath = format!("{}/{}.png", folder, date_to_string(date, "-", true));
    img::save_image(client, &url, &filepath).await?;

    print_step(progress, date, 3, format!("DONE: Saved to {filepath}"));

    Ok(())
}

fn print_step(progress: f32, date: NaiveDate, step: u32, status: String) {
    let progress = if step == 1 {
        let progress = format!("{:.2}", progress * 100.0);
        let progress = pad_left(&progress, 6, ' ');
        progress + "%"
    } else {
        String::from("       ")
    };

    let step = match step {
        1 => "1..",
        2 => " 2.",
        3 => "  3",
        _ => unreachable!("Invalid step"),
    };

    println!("   {progress}   {date}  [{step}]  {status}");
}

fn pad_left(text: &str, length: usize, ch: char) -> String {
    if text.len() > length {
        return text.to_string();
    }

    let pad = ch.to_string().repeat(length - text.len());

    pad + text
}
