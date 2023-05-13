use clap::Parser;

/// EveryGarf Downloader
///
/// Download every Garfield comic, to date
#[derive(Parser)]
#[command(author, version, about)]
pub struct Args {
    /// Folder to save images to
    pub folder: String,

    /// Clean (remove contents of) save folder (not recommended)
    #[arg(short, long, default_value_t = false)]
    pub clean: bool,

    /// Timeout for HTTP requests
    #[arg(short, long, default_value_t = 15)]
    pub timeout: u64,

    /// Don't send desktop notifications on error
    #[arg(short, long, default_value_t = false)]
    pub quiet: bool,

    /// Amount of fetch attempts allowed per thread, before hard error
    #[arg(short, long, default_value_t = 10)]
    pub attempts: u32,
}
