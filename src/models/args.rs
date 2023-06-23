use clap::Parser;

/// Simple program to convert a date to an ISO-8601 string.
/// Accepted formats:
/// "MM/DD/YYYY" ("01/01/1970"),
/// "MM/DD/YYYY HH:MM:SS" ("01/01/1970 12:00:00")
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// The date to format
    #[arg(index = 1)]
    pub date: Option<String>,
}
