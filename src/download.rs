use reqwest::Client;
use tokio::runtime::Runtime;

/// Download image, given url, and save to file
pub async fn save_image(client: &Client, url: &str, filepath: &str) -> Result<(), String> {
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
