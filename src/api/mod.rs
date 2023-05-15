mod slow;

use chrono::NaiveDate;
use reqwest::Client;

/// Fetch image and save to file
pub async fn fetch_and_save(
    client: &Client,
    date: NaiveDate,
    folder: &str,
    thread_no: usize,
    attempts: u32,
) -> Result<(), String> {
    // Attempt a limited number of times
    for i in 1..=attempts {
        let func = slow::fetch_and_save;

        match func(&client, date, folder, thread_no).await {
            // Success!
            Ok(()) => break,

            // Error
            Err(err) => {
                // Warn with attempt number
                eprintln!("\x1b[33m[warning] \x1b[2m[Attempt {i}]\x1b[0m Failed: {err}");
                // No more attempts - Return Error
                // Exits program from `main`
                if i >= attempts {
                    return Err(format!("Failed after {i} attempts: {err}"));
                }
            }
        }
    }

    Ok(())
}

// /// Attempt to fetch image and save to file
// ///
// /// Returns `Err` if anything fails
// async fn attempt_fetch_and_save(
//     client: &Client,
//     date: NaiveDate,
//     folder: &str,
//     thread_no: usize,
// ) -> Result<(), String> {
//     // Fetch image url, given date
//     print_step(date, thread_no, 1, format!("Fetching url of image"));
//     let url = url::fetch_url(client, date).await?;
//
//     // Download image, given url, and save to file, given filepath
//     print_step(
//         date,
//         thread_no,
//         2,
//         format!("Downloading image from \x1b[4m{url}\x1b[0m"),
//     );
//     let filepath = format!("{}/{}.png", folder, date_to_string(date, "-", true));
//     download::save_image(client, &url, &filepath).await?;
//
//     // Done!
//     print_step(
//         date,
//         thread_no,
//         3,
//         format!("Saved to \x1b[4m{filepath}\x1b[0m"),
//     );
//     Ok(())
// }

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
