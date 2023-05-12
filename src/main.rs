use std::{
    fs,
    path::Path,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use every_garfield::{
    date, date_from_filename, fetch_and_save_comic, filename_from_dir_entry, get_dates_between,
    get_parent_folder,
};
use reqwest::Client;

fn main() {
    println!("=== Every-Garfield ===");

    let folder = format!("{}/garfield", get_parent_folder().unwrap());

    println!("Save folder: {}", folder);

    if !Path::new(&folder).exists() {
        if let Err(err) = fs::create_dir(&folder) {
            panic!("Failed to create folder at `{folder}` - {err:?}");
        }
    }

    let date_first = date::first();
    let date_today = date::today();

    let all_dates = get_dates_between(date_first, date_today);

    let existing_dates = match fs::read_dir(&folder) {
        Ok(dir) => dir,
        Err(err) => panic!("Failed to read directory at `{folder}` - {err:?}"),
    };

    let existing_dates: Vec<_> = existing_dates
        .flatten()
        .filter_map(filename_from_dir_entry)
        .filter_map(|name| date_from_filename(&name))
        .collect();

    let mut missing_dates = Vec::new();

    for date in all_dates {
        if !existing_dates.contains(&date) {
            missing_dates.push(date);
        }
    }

    let job_count = missing_dates.len();
    let num_threads = num_cpus::get().min(job_count);

    if job_count < 1 {
        println!("Complete! No images missing to download!");
        return;
    }

    println!(
        "Downloading {} images using {} threads...",
        job_count, num_threads
    );

    let mut handles = Vec::new();
    let job_no = Arc::new(Mutex::new(0));

    for chunk in missing_dates.chunks(missing_dates.len() / num_threads + 1) {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("Failed to build request client");

        let chunk = chunk.to_vec();

        let folder = folder.clone();
        let job_no = job_no.clone();

        let handle = thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();

            rt.block_on(async move {
                for date in chunk {
                    let mut job_no = job_no.lock().unwrap();

                    let progress = *job_no as f32 / job_count as f32;
                    *job_no += 1;

                    if let Err(()) = fetch_and_save_comic(&client, date, &folder, progress).await {
                        eprintln!("FAILED. Refer to logs for details.");
                        std::process::exit(1);
                    }
                }
            });
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Complete! Downloaded {} images", job_count);
}
