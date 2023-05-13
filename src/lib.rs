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
    date: NaiveDate,
    folder: &str,
    thread_no: usize,
) -> Result<(), String> {
    //TODO Move to ouside loop
    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .map_err(|err| format!("Failed to build request client - {err:?}"))
        .unwrap();

    const ATTEMPTS: u32 = 3;

    for i in 1..=ATTEMPTS {
        match attempt_fetch_and_save(&client, date, folder, thread_no).await {
            Ok(()) => break,

            Err(err) => {
                eprintln!("[warning] [Attempt {i}] Failed: {err}");
                if i >= ATTEMPTS {
                    return Err(format!("Failed after {i} attempts: {err}"));
                }
            }
        }
    }

    Ok(())
}

pub async fn attempt_fetch_and_save(
    client: &Client,
    date: NaiveDate,
    folder: &str,
    thread_no: usize,
) -> Result<(), String> {
    print_step(date, thread_no, 1, format!("Fetching url of image"));

    let url = garf::comic_url(client, date).await?;

    print_step(date, thread_no, 2, format!("Fetching image from {url}"));

    let filepath = format!("{}/{}.png", folder, date_to_string(date, "-", true));
    img::save_image(client, &url, &filepath).await?;

    print_step(date, thread_no, 3, format!("DONE: Saved to {filepath}"));

    Ok(())
}

fn print_step(date: NaiveDate, thread_no: usize, step: u32, status: String) {
    let thread_no = (thread_no + 1).to_string() + if thread_no < 9 { " " } else { "" };

    let step = match step {
        1 => "1..",
        2 => " 2.",
        3 => "  3",
        _ => unreachable!("Invalid step"),
    };

    println!("    {date}  #{thread_no}  [{step}]  {status}");
}
