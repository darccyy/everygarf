use chrono::{Datelike, NaiveDate, Weekday};
use reqwest::Client;

use super::{print_step, save_image};

pub fn fetch_and_save(
    client: &Client,
    date: NaiveDate,
    filepath: &str,
    thread_no: usize,
) -> Result<(), String> {
    let url = format!(
        "https://picayune.uclick.com/comics/ga/{YYYY}/ga{YY}{MM}{DD}.{format}",
        YYYY = date.year(),
        YY = last_two_digits(date.year()),
        MM = leading_zero(date.month()),
        DD = leading_zero(date.day()),
        format = if date.weekday() == Weekday::Sun {
            "jpg"
        } else {
            "gif"
        },
    );

    print_step(
        date,
        thread_no,
        2,
        format!("Downloading image from \x1b[4m{url}\x1b[0m"),
    );

    save_image(client, &url, &filepath)?;

    // Done!
    print_step(
        date,
        thread_no,
        3,
        format!("Saved to \x1b[4m{filepath}\x1b[0m"),
    );

    Ok(())
}

fn last_two_digits(number: i32) -> String {
    leading_zero((number % 100) as u32)
}

fn leading_zero(number: u32) -> String {
    if number < 10 {
        String::from("0") + &number.to_string()
    } else {
        number.to_string()
    }
}
