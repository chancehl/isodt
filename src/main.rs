use chrono::{DateTime, Local, TimeZone, Utc};
use clap::Parser;
use models::args::Args;
use std::process;

mod models;

fn main() {
    let args = Args::parse();

    let date_format = "%Y-%m-%d".to_string();
    let date_time_format = "%Y-%m-%d %H:%M:%S".to_string();

    if let Some(date) = &args.date {
        println!(
            "{:?}",
            Utc.datetime_from_str(date, &date_format).expect(
                &format!(
                    "Could not convert date to ISO-8601 format. \
                    Please use the {:?} format for dates and the {:?} format for dates with time.",
                    date_format, date_time_format
                )
                .to_string()
            )
        )
    } else {
    };

    println!("{:?}", Utc::now());
    process::exit(0);
}
