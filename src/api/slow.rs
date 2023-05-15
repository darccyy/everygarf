use reqwest::Client;
use tokio::runtime::Runtime;

use chrono::NaiveDate;

use super::print_step;
use crate::date_to_string;

/// Fetch image and download
pub async fn fetch_and_save(
    client: &Client,
    date: NaiveDate,
    folder: &str,
    thread_no: usize,
) -> Result<(), String> {
    // Fetch image url, given date
    print_step(date, thread_no, 1, format!("Fetching url of image"));
    let url = fetch_url(client, date).await?;

    // Download image, given url, and save to file, given filepath
    print_step(
        date,
        thread_no,
        2,
        format!("Downloading image from \x1b[4m{url}\x1b[0m"),
    );
    let filepath = format!("{}/{}.png", folder, date_to_string(date, "-", true));
    save_image(client, &url, &filepath).await?;

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
async fn fetch_url(client: &Client, date: NaiveDate) -> Result<String, String> {
    // Convert date to YYYY/MM/DD string
    // Does not include trailing zero
    let date_string = date_to_string(date, "/", false);

    // Get webpage url from date string
    let url = format!(
        "https://corsproxy.garfieldapp.workers.dev/cors-proxy?https://www.gocomics.com/garfield/{}",
        date_string
    );

    // Fetch webpage body
    let response_body = Runtime::new().expect("Create runtime").block_on(async {
        client
            .get(url)
            .send()
            .await
            .map_err(|err| format!("Fetching webpage body for image url: {err}"))?
            .text()
            .await
            .map_err(|err| format!("Converting webpage body for image url to text: {err}"))
    })?;

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

/// Download image, given url, and save to file
async fn save_image(client: &Client, url: &str, filepath: &str) -> Result<(), String> {
    // Use tokio runtime
    // Requests and I/O cannot be performed without this
    Runtime::new().expect("Create runtime").block_on(async {
        // Fetch response
        let response = client
            .get(url)
            .send()
            .await
            .map_err(|err| format!("Fetching image from url: {err}"))?;

        // Get bytes of image
        let bytes = response
            .bytes()
            .await
            .map_err(|err| format!("Converting image to bytes: {err}"))?;

        // Parse image from bytes
        let image = image::load_from_memory(&bytes)
            .map_err(|err| format!("Loading image from bytes: {err}"))?;

        // Save image to file
        image
            .save(filepath)
            .map_err(|err| format!("Saving image file: {err}"))?;

        Ok(())
    })
}
