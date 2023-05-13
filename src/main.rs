use std::{
    fs,
    path::Path,
    sync::{Arc, Mutex},
    time::Duration,
};

use every_garfield::{
    date, date_from_filename, fetch_and_save_comic, filename_from_dir_entry, get_dates_between,
    get_parent_folder,
};
use futures::executor::block_on;
use notify_rust::Notification;

#[tokio::main]
async fn main() {
    if let Err(err) = run().await {
        eprintln!("[ERROR] {err}");

        Notification::new()
            .summary("EveryGarfield Failed")
            .body(&format!("Download failed.\n{err}"))
            .timeout(Duration::from_secs(5))
            .show()
            .expect("Failed to show notification");

        std::process::exit(1);
    }
}

async fn run() -> Result<(), String> {
    println!("=== EveryGarfield ===");

    let folder = format!("{}/garfield", get_parent_folder().unwrap());

    println!("Save folder: {}", folder);

    if !Path::new(&folder).exists() {
        fs::create_dir(&folder)
            .map_err(|err| format!("Failed to create folder at `{folder}` - {err:?}"))?;
    }

    let date_first = date::first();
    let date_today = date::today();

    let all_dates = get_dates_between(date_first, date_today);

    let existing_dates = fs::read_dir(&folder)
        .map_err(|err| format!("Failed to read directory at `{folder}` - {err:?}"))?;

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
        return Ok(());
    }

    println!(
        "Downloading {} images using {} threads...",
        job_count, num_threads
    );

    let mut threads = vec![];
    let job_no = Arc::new(Mutex::new(0));

    for (_thread_no, chunk) in missing_dates
        .chunks(job_count / num_threads + 1)
        .enumerate()
    {
        let chunk = chunk.to_vec();

        let job_no = Arc::clone(&job_no);

        let handle = std::thread::spawn(move || {
            for date in chunk.into_iter() {
                let mut job_no = job_no.lock().unwrap();
                let progress = *job_no as f32 / job_count as f32 * 100.0;
                *job_no += 1;

                let job = fetch_and_save_comic(date, "/home/darcy/Pictures/garfield", progress);

                let result = block_on(job);

                if let Err(err) = result {
                    return Err(err);
                }
            }
            Ok(())
        });

        threads.push(handle);
    }

    for handle in threads {
        handle.join().unwrap()?;
    }

    println!("Complete! Downloaded {} images", job_count);
    Ok(())
}
