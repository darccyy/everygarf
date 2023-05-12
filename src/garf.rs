use chrono::NaiveDate;
use reqwest::Client;

use super::date_to_string;

/// Get image URL of comic, asynchronously, given a date (`NaiveDate`)
pub async fn comic_url(client: &Client, date: NaiveDate) -> Result<String, String> {
    // Convert date to YYYY/MM/DD string
    let date_string = date_to_string(date, "/", false);

    // Get webpage url from date string
    let url = format!(
        "https://corsproxy.garfieldapp.workers.dev/cors-proxy?https://www.gocomics.com/garfield/{}",
        date_string
    );

    // Fetch webpage body
    let response_body = client
        .get(url)
        .send()
        .await
        .map_err(|err| format!("Fetching webpage body for image url: {err}"))?
        .text()
        .await
        .map_err(|err| format!("Converting webpage body for image url to text: {err}"))?;

    // Find image url in HTML
    let Some(char_index)= response_body
        .find("https://assets.amuniversal.com")
        else {
            return Err(format!("Cannot find image url in webpage body"));
        };

    // Get string from character index
    let Some(image_url)= response_body.get(char_index..char_index + 63) else {
        return Err(format!("Slicing text of webpage body for image url"));
    };

    Ok(image_url.to_string())
}
