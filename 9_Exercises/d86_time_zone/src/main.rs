// Day 86: Project: Time Zone Converter CLI (with chrono-tz)
// Build a CLI tool that converts datetime strings between time zones using the chrono and 
// chrono-tz crates. You’ll learn to parse strings, convert them across zones, and handle 
// user input for global time conversions.
// Key Concepts:
// + chrono::NaiveDateTime for parsing local times
// + chrono_tz::Tz to handle named time zones
// + from_local_datetime and .with_timezone() for conversion
// You now have a useful CLI utility for converting datetimes between any of the IANA time 
// zones, ideal for travelers, remote teams, and scheduling tools.
use chrono::{DateTime, NaiveDateTime, TimeZone};
use chrono_tz::Tz;
use std::io::{self, Write};

fn main() {
    println!("Time Zone Converter!");

    // Step 1: Ask for input datetime
    let datetime_input = prompt("Enter datetime (YYYY-MM-DD HH:MM): ");

    let src_tz_input =  prompt("Source timezone (e.g. UTC, America/New_York): ");

    let tgt_tz_input = prompt("Target timezone (e.g. Asia/Tokyo): ");

    // Step 3: Parse everything
    let naive = NaiveDateTime::parse_from_str(datetime_input.as_str(), "%Y-%m-%d %H:%M");
    let src_tz: Result<Tz, _> = src_tz_input.parse();
    let tgt_tz: Result<Tz, _> = tgt_tz_input.parse();

    match (naive, src_tz, tgt_tz) {
        (Ok(naive_dt), Ok(src_tz), Ok(tgt_tz)) => {
            let src_dt: DateTime<Tz> = src_tz.from_local_datetime(&naive_dt).single().unwrap();
            let tgt_dt: DateTime<Tz> = src_dt.with_timezone(&tgt_tz);
 
            println!("\n{} in {} → {} in {}", naive_dt, src_tz, tgt_dt.format("%Y-%m-%d %H:%M"), tgt_tz);
        }
        _ => {
            println!("Invalid input. Please check your datetime or timezone formats.");
        }
    }
    
}

fn prompt(msg: &str) -> String {
    print!("{}", msg);
    io::stdout().flush().unwrap();
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim().to_string()
}