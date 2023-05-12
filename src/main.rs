use std::{fs, thread};

use every_garfield::{
    date, date_from_filename, fetch_and_save_comic, filename_from_dir_entry, get_dates_between,
};
use reqwest::Client;

fn main() {
    let folder = fs::read_to_string("./folder").unwrap().trim().to_string();

    let date_first = date::first();
    let date_today = date::today();

    let all_dates = get_dates_between(date_first, date_today);

    let existing_dates: Vec<_> = fs::read_dir(&folder)
        .unwrap()
        .flatten()
        .filter_map(filename_from_dir_entry)
        .filter_map(|name| date_from_filename(&name))
        .collect();

    if existing_dates.len() >= all_dates.len() {
        println!("None missing!");
        return;
    }

    let mut missing_dates = Vec::new();

    for date in all_dates {
        if !existing_dates.contains(&date) {
            missing_dates.push(date);
        }
    }

    let job_count = missing_dates.len();
    let num_threads = num_cpus::get().min(job_count);

    println!(
        "Downloading {} images using {} threads...",
        job_count, num_threads
    );

    let mut handles = Vec::new();

    for (_thread_no, chunk) in missing_dates
        .chunks(missing_dates.len() / num_threads + 1)
        .enumerate()
    {
        let client = Client::new();
        let chunk = chunk.to_vec();

        let folder = folder.clone();

        let handle = thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();

            rt.block_on(async move {
                for date in chunk {
                    fetch_and_save_comic(&client, date, &folder).await.unwrap();
                }
            });
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Complete!\nDownloaded {} images", job_count);
}
