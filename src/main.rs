use std::{fs, path::Path, time::Duration};

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

    println!("Checking for missing images in: {}/", folder);

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
    let thread_count = num_cpus::get().min(job_count);

    if job_count < 1 {
        println!("Complete! No images missing to download!");
        return Ok(());
    }

    println!(
        "Downloading {} images using (up to) {} threads...",
        job_count, thread_count
    );

    let chunk_size = job_count / thread_count + 1;
    let mut threads = vec![];

    for (thread_no, chunk) in missing_dates.chunks(chunk_size).enumerate() {
        let chunk = chunk.to_vec();

        let handle = std::thread::spawn(move || {
            for date in chunk {
                let job = fetch_and_save_comic(date, "/home/darcy/Pictures/garfield", thread_no);

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
