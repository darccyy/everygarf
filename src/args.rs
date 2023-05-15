use clap::Parser;

/// EveryGarf Comic Downloader
///
/// Download every Garfield comic, to date
#[derive(Parser)]
#[command(author, version, about)]
pub struct Args {
    /// Folder to download images into
    ///
    /// Leave blank to use 'garfield' folder in user pictures directory (~/Pictures/garfield)
    pub folder: Option<String>,

    /// Don't send desktop notifications on error
    #[arg(short, long)]
    pub quiet: bool,

    /// Clean (remove contents of) save folder (not recommended)
    #[arg(short, long)]
    pub clean: bool,

    /// Timeout for HTTP requests
    #[arg(short, long, default_value_t = 15)]
    pub timeout: u64,

    /// Amount of fetch attempts allowed per thread, before hard error
    #[arg(short, long, default_value_t = 10)]
    pub attempts: u32,

    /// Max CPU threads to use
    ///
    /// By default uses all threads possible
    #[arg(short, long)]
    pub max_threads: Option<usize>,

    /// Use alternative API
    ///
    /// Faster, but less reliable, unmaintained - DO NOT EXPECT THIS TO WORK!
    ///
    /// See README for more information
    #[arg(long)]
    pub alt_api: bool,
}
