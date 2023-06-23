use clap::Parser;

/// Simple program to convert a date to an ISO-8601 string
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// The date to format
    #[arg(index = 1)]
    pub date: Option<String>,
}
