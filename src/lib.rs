/// Get all dates between first comic and today
mod date;
/// Download image, given url, and save to file
mod download;
/// Fetch image and save to file
mod fetch;
/// Fetch image url, given date
mod url;

pub use date::get_all_dates;
pub use fetch::fetch_and_save;

use chrono::{Datelike, NaiveDate};
use std::fs::{self, DirEntry};

/// Canonicalize folder, replace `~` with home directory
pub fn parse_folder_path(mut folder: String) -> Result<String, String> {
    // Use home directory shorthand
    if folder.starts_with("~/") {
        // Get home directory
        let Some(Some(home)) = dirs_next::home_dir().map(|dir|dir.to_str().map(|dir|dir.to_string())) else {
            return Err(format!("Home directory cannot be found. Try entering manually with `/home/...`"));
        };

        // Remove first character
        let mut chars = folder.chars();
        chars.next();

        // Concatenate
        folder = home + chars.as_str();
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

/// Converts `DirEntry` into `String`
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
