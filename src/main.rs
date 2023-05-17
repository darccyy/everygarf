/// CLI Arguments
mod args;

use std::{
    fs,
    sync::Arc,
    time::{Duration, Instant},
};

use clap::Parser;
use human_bytes::human_bytes;
use humantime::format_duration;
use notify_rust::Notification;
use reqwest::blocking::Client;

use args::Args;
use everygarf::{
    date_from_filename, fetch_and_save, filename_from_dir_entry, get_all_dates, get_folder_path,
};

fn main() {
    // Parse CLI arguments
    let args = Args::parse();

    // Run whole program
    let result = if args.attempts > 0 {
        run_all(&args)
    } else {
        // CLI argument `attempts` is zero
        Err(String::from("No attempts were allowed :("))
    };

    // Global error downloading
    // Mainly due to network or IO
    if let Err(err) = result {
        eprintln!();
        eprintln!("\x1b[31m=============[ERROR]=============\x1b[0m");
        eprintln!("{err}");
        eprintln!("\x1b[31m=================================\x1b[0m");

        // Send desktop notification
        if !args.quiet {
            Notification::new()
                .summary("EveryGarf Failed")
                .body(&format!("Download failed.\n{err}"))
                .timeout(Duration::from_secs(10))
                .show()
                .expect("Failed to show notification");
        }

        std::process::exit(1);
    }
}

/// Run whole program
fn run_all(args: &Args) -> Result<(), String> {
    let start_time = Instant::now();

    println!();
    println!("\x1b[1m ┌─────────────┐\x1b[0m");
    println!("\x1b[1m │  EveryGarf  │\x1b[0m");
    println!("\x1b[1m └─────────────┘ \x1b[0;3mComic Downloader\x1b[0m");

    // Get folder location
    let folder = get_folder_path(args.folder.to_owned())?;

    // Run download
    let download_count = run_download(folder.clone(), args)?;

    // Show time program took to complete
    let elapsed_time = format_duration(Duration::from_secs(start_time.elapsed().as_secs()));
    // Get size of folder
    let folder_size = match fs_extra::dir::get_size(folder) {
        Ok(size) => human_bytes(size as f64),
        Err(_) => String::from("[error]"),
    };

    println!();
    println!("\x1b[1;32mComplete!\x1b[0m");
    println!(
        " \x1b[2m•\x1b[0m Downloaded:   \x1b[1m{} files\x1b[0m",
        download_count,
    );
    println!(
        " \x1b[2m•\x1b[0m Elapsed time: \x1b[1m{}\x1b[0m",
        elapsed_time,
    );
    println!(
        " \x1b[2m•\x1b[0m Total size:   \x1b[1m{}\x1b[0m",
        folder_size,
    );
    println!();

    Ok(())
}

/// Run only the download
fn run_download(folder: String, args: &Args) -> Result<usize, String> {
    // Clean folder if argument given
    if args.clean {
        println!("Removing all images in: \x1b[4m{folder}\x1b[0m");

        // Remove folder recursively
        fs::remove_dir_all(&folder)
            .map_err(|err| format!("Failed to remove folder `{folder}` - {err:?}"))?;

        // Create folder again
        // Does not create parent folders iteratively
        fs::create_dir(&folder)
            .map_err(|err| format!("Failed to re-create folder `{folder}` - {err:?}"))?;
    } else {
        println!("Checking for missing images in: \x1b[4m{folder}\x1b[0m");
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
    let mut thread_count = num_cpus::get().min(job_count);
    if let Some(max_threads) = args.max_threads {
        thread_count = thread_count.min(max_threads);
    }

    // No images are missing
    if job_count < 1 {
        println!("\x1b[32mEverything is already up to date!\x1b[0m");
        return Ok(0);
    }

    println!(
        "Downloading \x1b[1m{}\x1b[0m images using (up to) \x1b[1m{}\x1b[0m threads...\n\x1b[2mNote: Downloads are not in order\x1b[0m",
        job_count, thread_count,
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

        // Copy argument values
        let timeout = args.timeout;
        let attempts = args.attempts;
        let alt_api = args.alt_api;

        // Spawn thread and add to list
        let handle = std::thread::spawn(move || {
            // Create http client (one per thread)
            // Timeout from cli argument
            let client = Client::builder()
                .timeout(std::time::Duration::from_secs(timeout))
                .build()
                .map_err(|err| format!("Failed to build request client - {err:?}"))
                .unwrap();

            // Run jobs per thread
            for date in chunk {
                // Fetch image from date, and save to folder
                let result = fetch_and_save(&client, date, &folder, thread_no, attempts, alt_api);

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
    Ok(job_count)
}
