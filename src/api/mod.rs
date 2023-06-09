mod fast;
mod slow;

use chrono::NaiveDate;
use reqwest::blocking::Client;

use crate::date_to_string;

/// Fetch image and save to file
pub fn fetch_and_save(
    client: &Client,
    date: NaiveDate,
    folder: &str,
    thread_no: usize,
    attempts: u32,
    alt_api: bool,
) -> Result<(), String> {
    let filepath = format!("{}{}.png", folder, date_to_string(date, "-", true));

    // Attempt a limited number of times
    for i in 1..=attempts {
        let result = if !alt_api {
            // Slow, but reliable and robust
            slow::fetch_and_save(&client, date, &filepath, thread_no)
        } else {
            // Fast, but unreliable
            fast::fetch_and_save(&client, date, &filepath, thread_no)
        };

        match result {
            // Success!
            Ok(()) => break,

            // Error
            Err(err) => {
                // Warn with attempt number
                eprintln!("\x1b[33m[warning] \x1b[2m[Attempt {i}]\x1b[0m \x1b[1m{date}\x1b[0m Failed: {err}");
                // No more attempts - Return Error
                // Exits program from `main`
                if i >= attempts {
                    return Err(format!(
                        "\x1b[1m{date}\x1b[0m Failed after {i} attempts: {err}"
                    ));
                }
            }
        }
    }

    Ok(())
}

/// Print information for current stop of job
///
/// Date of image, thread number, step number, and status information
fn print_step(date: NaiveDate, thread_no: usize, step: u32, info: String) {
    // Add leading zero
    let thread_no = (thread_no + 1).to_string() + if thread_no < 9 { " " } else { "" };

    // Create tick icon
    let icon = if step == 3 { "\x1b[32m✓\x1b[0m" } else { " " };

    // Make fancy
    let step = format!(
        "{}{step}\x1b[2m{}\x1b[0;34m",
        " ".repeat(step.max(1) as usize - 1),
        "•".repeat(3 - step.min(3) as usize),
    );

    println!("    \x1b[1m{date}\x1b[0m  \x1b[2m#{thread_no}\x1b[0m  \x1b[34m[{step}]\x1b[0m {icon} {info}");
}

/// Download image, given url, and save to file
fn save_image(client: &Client, url: &str, filepath: &str) -> Result<(), String> {
    // Fetch response
    let response = client
        .get(url)
        .send()
        .map_err(|err| format!("Fetching image from url ({url}) - {err}"))?;

    // Get bytes of image
    let bytes = response
        .bytes()
        .map_err(|err| format!("Converting image to bytes ({url}) - {err}"))?;

    // Parse image from bytes
    let image = image::load_from_memory(&bytes)
        .map_err(|err| format!("Loading image from bytes ({url}) - {err}"))?;

    // Save image to file
    image
        .save(filepath)
        .map_err(|err| format!("Saving image file ({filepath}) - {err}"))?;

    Ok(())
}
