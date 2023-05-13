use chrono::NaiveDate;
use reqwest::Client;

use super::{date_to_string, download, url};

pub async fn fetch_and_save(date: NaiveDate, folder: &str, thread_no: usize) -> Result<(), String> {
    //TODO Move to ouside loop
    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .map_err(|err| format!("Failed to build request client - {err:?}"))
        .unwrap();

    const ATTEMPTS: u32 = 3;

    for i in 1..=ATTEMPTS {
        match attempt_fetch_and_save(&client, date, folder, thread_no).await {
            Ok(()) => break,

            Err(err) => {
                eprintln!("[warning] [Attempt {i}] Failed: {err}");
                if i >= ATTEMPTS {
                    return Err(format!("Failed after {i} attempts: {err}"));
                }
            }
        }
    }

    Ok(())
}

pub async fn attempt_fetch_and_save(
    client: &Client,
    date: NaiveDate,
    folder: &str,
    thread_no: usize,
) -> Result<(), String> {
    print_step(date, thread_no, 1, format!("Fetching url of image"));

    let url = url::fetch_url(client, date).await?;

    print_step(date, thread_no, 2, format!("Fetching image from {url}"));

    let filepath = format!("{}/{}.png", folder, date_to_string(date, "-", true));
    download::save_image(client, &url, &filepath).await?;

    print_step(date, thread_no, 3, format!("DONE: Saved to {filepath}"));

    Ok(())
}

fn print_step(date: NaiveDate, thread_no: usize, step: u32, status: String) {
    let thread_no = (thread_no + 1).to_string() + if thread_no < 9 { " " } else { "" };

    let step = match step {
        1 => "1..",
        2 => " 2.",
        3 => "  3",
        _ => unreachable!("Invalid step"),
    };

    println!("    {date}  #{thread_no}  [{step}]  {status}");
}
