use chrono::NaiveDate;
use reqwest::Client;

use super::{date_to_string, download, url};

/// Fetch image and save to file
pub async fn fetch_and_save(
    client: &Client,
    date: NaiveDate,
    folder: &str,
    thread_no: usize,
) -> Result<(), String> {
    // TODO Move to cli arguemnt
    const ATTEMPTS: u32 = 3;

    // Attempt a limited number of times
    for i in 1..=ATTEMPTS {
        match attempt_fetch_and_save(&client, date, folder, thread_no).await {
            // Success!
            Ok(()) => break,

            // Error
            Err(err) => {
                // Warn with attempt number
                eprintln!("[warning] [Attempt {i}] Failed: {err}");
                // No more attempts - Return Error
                // Exits program from `main`
                if i >= ATTEMPTS {
                    return Err(format!("Failed after {i} attempts: {err}"));
                }
            }
        }
    }

    Ok(())
}

/// Attempt to fetch image and save to file
///
/// Returns `Err` if anything fails
async fn attempt_fetch_and_save(
    client: &Client,
    date: NaiveDate,
    folder: &str,
    thread_no: usize,
) -> Result<(), String> {
    // Fetch image url, given date
    print_step(date, thread_no, 1, format!("Fetching url of image"));
    let url = url::fetch_url(client, date).await?;

    // Download image, given url, and save to file, given filepath
    print_step(date, thread_no, 2, format!("Downloading image from {url}"));
    let filepath = format!("{}/{}.png", folder, date_to_string(date, "-", true));
    download::save_image(client, &url, &filepath).await?;

    // Done!
    print_step(date, thread_no, 3, format!("DONE: Saved to {filepath}"));
    Ok(())
}

/// Print information for current stop of job
///
/// Date of image, thread number, step number, and status information
fn print_step(date: NaiveDate, thread_no: usize, step: u32, info: String) {
    // Add leading zero
    let thread_no = (thread_no + 1).to_string() + if thread_no < 9 { " " } else { "" };

    // Make fancy
    let step = match step {
        1 => "1..",
        2 => " 2.",
        3 => "  3",
        _ => unreachable!("Invalid step"),
    };

    println!("    {date}  #{thread_no}  [{step}]  {info}");
}
