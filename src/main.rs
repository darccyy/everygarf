use std::{
    fs,
    path::Path,
    sync::Arc,
    time::{Duration, Instant},
};

use futures::executor::block_on;
use humantime::format_duration;
use notify_rust::Notification;

use every_garfield::{
    date_from_filename, fetch_and_save, filename_from_dir_entry, get_all_dates, get_parent_folder,
};

#[tokio::main]
async fn main() {
    let start_time = Instant::now();

    let result = run().await;

    if let Err(err) = result {
        eprintln!("[ERROR] {err}");

        Notification::new()
            .summary("EveryGarfield Failed")
            .body(&format!("Download failed.\n{err}"))
            .timeout(Duration::from_secs(5))
            .show()
            .expect("Failed to show notification");

        std::process::exit(1);
    }

    let elapsed_time = Duration::from_secs(start_time.elapsed().as_secs());
    println!("Elapsed time: {}", format_duration(elapsed_time));
}

async fn run() -> Result<(), String> {
    println!("=== EveryGarfield ===");

    let folder = format!("{}/garfield", get_parent_folder().unwrap());

    println!("Checking for missing images in: {}/", folder);

    if !Path::new(&folder).exists() {
        fs::create_dir(&folder)
            .map_err(|err| format!("Failed to create folder at `{folder}` - {err:?}"))?;
    }

    let all_dates = get_all_dates();

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

    let folder = Arc::new(folder);

    for (thread_no, chunk) in missing_dates.chunks(chunk_size).enumerate() {
        let chunk = chunk.to_vec();

        let folder = Arc::clone(&folder);

        let handle = std::thread::spawn(move || {
            for date in chunk {
                let job = fetch_and_save(date, &folder, thread_no);

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
