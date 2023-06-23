use chrono::{NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc};
use clap::Parser;
use models::args::Args;
use regex::Regex;
use std::process;

mod models;

fn main() {
    let args = Args::parse();

    let date_format = "%m/%d/%Y".to_string();
    let date_regex = Regex::new("^\\d{2}\\/\\d{2}\\/\\d{4}$").unwrap();

    let date_time_format = "%m/%d/%Y %H:%M:%S".to_string();
    let date_time_regex =
        Regex::new("^\\d{2}\\/\\d{2}\\/\\d{4}\\s{1}\\d{2}:\\d{2}:\\d{2}$").unwrap();

    if let Some(date) = &args.date {
        if date_time_regex.is_match(date) {
            println!(
                "{:?}",
                Utc.datetime_from_str(date, &date_time_format)
                    .expect("Could not parse date time from date")
            )
        } else if date_regex.is_match(date) {
            let naive_date =
                NaiveDate::parse_from_str(date, &date_format).expect("Could not parse date");

            let naive_time = NaiveTime::from_hms_opt(0, 0, 0)
                .expect("Could not compute a naive time from hms values");

            let naive_date_time = NaiveDateTime::new(naive_date, naive_time);

            let fixed_offset = naive_date_time.and_utc().fixed_offset();

            let utc = Utc
                .from_local_datetime(&fixed_offset.naive_utc())
                .single()
                .expect("Could not compute UTC time from naively constructed date time");

            println!("{:?}", utc);
        } else {
            panic!("Date {:?} did not match accepted formats. Please run \"isodt --help\" for a list of accepted formats", date);
        }
    } else {
        println!("{:?}", Utc::now());
        process::exit(0);
    };
}
