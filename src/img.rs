use reqwest::Client;

pub async fn save_image(client: &Client, url: &str, filepath: &str) -> Result<(), ()> {
    // Fetch response
    let response = client.get(url).send().await.unwrap();

    // Get bytes
    let bytes = response.bytes().await.unwrap();

    // Parse bytes as image
    let image = image::load_from_memory(&bytes).unwrap();

    image.save(filepath).unwrap();

    Ok(())
}
