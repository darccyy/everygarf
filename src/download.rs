use reqwest::Client;
use tokio::runtime::Runtime;

pub async fn save_image(client: &Client, url: &str, filepath: &str) -> Result<(), String> {
    Runtime::new().expect("Create runtime").block_on(async {
        // Fetch response
        let response = client
            .get(url)
            .send()
            .await
            .map_err(|err| format!("Fetching image from url: {err}"))?;

        // Get bytes
        let bytes = response
            .bytes()
            .await
            .map_err(|err| format!("Converting image to bytes: {err}"))?;

        // Parse bytes as image
        let image = image::load_from_memory(&bytes)
            .map_err(|err| format!("Loading image from bytes: {err}"))?;

        // return Err(format!("IDK!"));

        image
            .save(filepath)
            .map_err(|err| format!("Saving image file: {err}"))?;

        Ok(())
    })
}
