/// CLI Arguments
mod args;

use std::{
    fs,
    sync::Arc,
    time::{Duration, Instant},
};

use clap::Parser;
use futures::executor::block_on;
use humantime::format_duration;
use notify_rust::Notification;

use args::Args;
use everygarf::{
    date_from_filename, fetch_and_save, filename_from_dir_entry, get_all_dates, parse_folder_path,
};
use reqwest::Client;

#[tokio::main]
async fn main() {
    // Parse CLI arguments
    let args = Args::parse();

    println!("=== EveryGarf ===");
    let start_time = Instant::now();

    // Error downloading
    // Due to network or IO
    if let Err(err) = run(args).await {
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

async fn run(args: Args) -> Result<(), String> {
    // Parse folder path from user input
    let folder = parse_folder_path(args.folder)?;

    // Clean folder if argument given
    if args.clean {
        println!("Removing all images in: {folder}");

        // Remove folder recursively
        fs::remove_dir_all(&folder)
            .map_err(|err| format!("Failed to remove folder `{folder}` - {err:?}"))?;

        // Create folder again
        // Does not create parent folders iteratively
        fs::create_dir(&folder)
            .map_err(|err| format!("Failed to re-create folder `{folder}` - {err:?}"))?;
    } else {
        println!("Checking for missing images in: {folder}");
    }

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
            // Create http client (one per thread)
            // Timeout from cli argument
            let client = Client::builder()
                .timeout(std::time::Duration::from_secs(args.timeout))
                .build()
                .map_err(|err| format!("Failed to build request client - {err:?}"))
                .unwrap();

            // Run jobs per thread
            for date in chunk {
                // Fetch image from date, and save to folder
                let job = fetch_and_save(&client, date, &folder, thread_no);

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
