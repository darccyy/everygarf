use chrono::NaiveDate;
use reqwest::blocking::Client;

use super::{print_step, save_image};
use crate::date_to_string;

/// Fetch image and download
pub fn fetch_and_save(
    client: &Client,
    date: NaiveDate,
    filepath: &str,
    thread_no: usize,
) -> Result<(), String> {
    print_step(date, thread_no, 1, format!("Fetching url of image"));

    // Fetch image url, given date
    let url = fetch_url(client, date)?;

    print_step(
        date,
        thread_no,
        2,
        format!("Downloading image from \x1b[4m{url}\x1b[0m"),
    );

    // Download image, given url, and save to file, given filepath
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

/// Fetch image url, given date
fn fetch_url(client: &Client, date: NaiveDate) -> Result<String, String> {
    // Convert date to YYYY/MM/DD string
    // Does not include trailing zero
    let date_string = date_to_string(date, "/", false);

    // Get webpage url from date string
    let url = format!(
        "https://corsproxy.garfieldapp.workers.dev/cors-proxy?https://www.gocomics.com/garfield/{}",
        date_string
    );

    // Fetch request
    let response= client
        .get(&url)
        .send()
        .map_err(|err| format!("Fetching webpage body for image url ({url}) - {err}"))?
        .error_for_status()
        .map_err(|err| format!("Server returned error ({url}) Possibly rate limited by Cloudflare. Try again in a few minutes. - {err}"))?;

    // Fetch webpage body
    let response_body = response
        .text()
        .map_err(|err| format!("Converting webpage body for image url to text ({url}) - {err}"))?;

    // Find image url in HTML
    let Some(char_index)= response_body
        .find("https://assets.amuniversal.com")
        else {
            return Err(format!("Cannot find image url in webpage body ({url})"));
        };

    // Get string from character index
    let Some(image_url)= response_body.get(char_index..char_index + 63) else {
        return Err(format!("Slicing text of webpage body for image url ({url})"));
    };

    Ok(image_url.to_string())
}
