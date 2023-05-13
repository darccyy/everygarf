use std::{
    fs,
    path::Path,
    sync::Arc,
    time::{Duration, Instant},
};

use futures::executor::block_on;
use humantime::format_duration;
use notify_rust::Notification;

use everygarf::{
    date_from_filename, fetch_and_save, filename_from_dir_entry, get_all_dates, get_parent_folder,
};

#[tokio::main]
async fn main() {
    println!("=== EveryGarf ===");
    let start_time = Instant::now();

    // Error downloading
    // Due to network or IO
    if let Err(err) = run().await {
        eprintln!("[ERROR] {err}");

        // Send desktop notification
        Notification::new()
            .summary("EveryGarf Failed")
            .body(&format!("Download failed.\n{err}"))
            .timeout(Duration::from_secs(5))
            .show()
            .expect("Failed to show notification");

        std::process::exit(1);
    }

    // Show time program took to complete
    let elapsed_time = Duration::from_secs(start_time.elapsed().as_secs());
    println!("Elapsed time: {}", format_duration(elapsed_time));
}

async fn run() -> Result<(), String> {
    let folder = format!("{}/garfield", get_parent_folder().unwrap());

    // Create folder if it does not already exist
    // Does not create parent folders iteratively
    if !Path::new(&folder).exists() {
        fs::create_dir(&folder)
            .map_err(|err| format!("Failed to create folder at `{folder}` - {err:?}"))?;
    }

    println!("Checking for missing images in: {}/", folder);

    // All dates that have a comic
    let all_dates = get_all_dates();

    // Read all files in folder
    let existing_files = fs::read_dir(&folder)
        .map_err(|err| format!("Failed to read directory at `{folder}` - {err:?}"))?;
    // Filter and map to get all existing dates in folder
    let existing_dates: Vec<_> = existing_files
        .flatten()
        .filter_map(filename_from_dir_entry)
        .filter_map(|name| date_from_filename(&name))
        .collect();

    // Get all possible dates, which are not already downloaded
    let missing_dates: Vec<_> = all_dates
        .into_iter()
        .filter(|date| !existing_dates.contains(&date))
        .collect();

    // Amount of images to download
    let job_count = missing_dates.len();
    // Max amount of threads to use
    let thread_count = num_cpus::get().min(job_count);

    // No images are missing
    if job_count < 1 {
        println!("Complete! No images missing to download!");
        return Ok(());
    }

    println!(
        "Downloading {} images using (up to) {} threads...",
        job_count, thread_count
    );

    // Number of jobs (images) per thread
    let chunk_size = job_count / thread_count + 1;
    // List of threads
    let mut threads = vec![];

    // Convert folder to atomic rc to be used immutably between threads
    let folder = Arc::new(folder);

    // Create threads
    for (thread_no, chunk) in missing_dates.chunks(chunk_size).enumerate() {
        let chunk = chunk.to_vec();
        let folder = Arc::clone(&folder);

        // Spawn thread and add to list
        let handle = std::thread::spawn(move || {
            // Run jobs per thread
            for date in chunk {
                // Fetch image from date, and save to folder
                let job = fetch_and_save(date, &folder, thread_no);

                // Block thread, while async function runs
                let result = block_on(job);

                // Stop thread and report error
                if let Err(err) = result {
                    return Err(err);
                }
            }
            // All jobs completed successfully
            Ok(())
        });

        threads.push(handle);
    }

    // Join threads
    for handle in threads {
        handle.join().unwrap()?;
    }

    // All jobs in all threads completed successfully
    println!("Complete! Downloaded {} images", job_count);
    Ok(())
}
