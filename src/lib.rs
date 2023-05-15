/// Fetch and download image
mod api;
/// Get all dates between first comic and today
mod date;

pub use api::fetch_and_save;
pub use date::get_all_dates;

use chrono::{Datelike, NaiveDate};
use std::{
    fs::{self, DirEntry},
    path::{Path, PathBuf},
};

/// Canonicalize folder, or find automatically, replace `~` with home directory
pub fn get_folder_path(folder: Option<String>) -> Result<String, String> {
    // Parse folder name, or use automatic
    let folder = match folder {
        // Use given folder
        Some(folder) => {
            // Using home directory shorthand
            if folder.starts_with("~/") {
                // Get home directory
                let Some(Some(home)) = dirs_next::home_dir().map(path_buf_to_string) else {
                    return Err(format!("Home directory cannot be found. Please enter manually with `/home/...`"));
                };

                // Remove first character
                let mut chars = folder.chars();
                chars.next();
                // Concatenate
                home + chars.as_str()
            } else {
                folder
            }
        },

        // Get parent folder automatically
        None => match get_auto_parent_folder() {
            Some(folder) => folder + "/garfield",
            None =>  return Err(format!("Cannot automatically find appropriate folder location. Please enter folder manually"))
        },
    };

    // Create folder if not exist
    // Does not create parents
    if !Path::new(&folder).exists() {
        fs::create_dir(&folder)
            .map_err(|err| format!("Failed to create folder `{folder}` - {err:?}"))?;
    }

    // Canonicalize directory
    let folder_path = fs::canonicalize(&folder)
        .map_err(|err| format!("Invalid folder path `{folder}` - {err:?}"))?;

    // Convert path to string
    let Some(folder) = folder_path.to_str() else {
        return Err(format!("Invalid folder path `{folder}` - Invalid string"));
    };
    let mut folder = folder.to_string();

    // Add `/` to end, if not already
    if !folder.ends_with('/') {
        folder.push('/');
    }

    Ok(folder)
}

/// Automatically get parent of download folder
///
/// Returns `None` if no appropriate folder could be found
fn get_auto_parent_folder() -> Option<String> {
    let dir = if let Some(dir) = dirs_next::picture_dir() {
        dir
    } else if let Some(dir) = dirs_next::document_dir() {
        dir
    } else if let Some(dir) = dirs_next::home_dir() {
        dir
    } else {
        return None;
    };

    path_buf_to_string(dir)
}

/// Easily convert `PathBuf` to String
///
/// Returns `None` if any string conversion fails
fn path_buf_to_string(path: PathBuf) -> Option<String> {
    path.to_str().map(|path| path.to_string())
}

/// Easily convert `DirEntry` into `String`
///
/// Returns `None` if any string conversion fails
pub fn filename_from_dir_entry(dir_entry: DirEntry) -> Option<String> {
    let name = dir_entry.file_name();
    Some(name.to_str()?.to_string())
}

/// Parse date from filename, as `NaiveDate`
///
/// Returns `None` if date is not found, or is incorrectly formatted
pub fn date_from_filename(filename: &str) -> Option<NaiveDate> {
    let name = filename.split('/').last()?.split('.').next()?;
    let mut parts = name.split('-');

    let year = parts.next()?;
    let month = parts.next()?;
    let day = parts.next()?;

    let year: i32 = year.parse().ok()?;
    let month: u32 = month.parse().ok()?;
    let day: u32 = day.parse().ok()?;

    NaiveDate::from_ymd_opt(year, month, day)
}

/// Convert `NaiveDate` to YYYY/MM/DD format, with '/' being a given string separator, and
/// optionally includes leading zeros for months and days (Eg. '03')
fn date_to_string(date: NaiveDate, separator: &str, leading_zeros: bool) -> String {
    let month = date.month();
    let day = date.day();

    date.year().to_string()
        + separator
        + if leading_zeros && month < 10 { "0" } else { "" }
        + &month.to_string()
        + separator
        + if leading_zeros && day < 10 { "0" } else { "" }
        + &day.to_string()
}
