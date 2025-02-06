use chrono::{DateTime, NaiveDateTime, TimeZone};
use chrono_tz::Tz;
use clap::Parser;
use std::str::FromStr; // Import the FromStr trait

#[derive(Parser)]
struct Cli {
    #[clap(default_value = "UTC", help = "Source timezone")]
    from_tz: String,
    #[clap(default_value = "Australia/Sydney", help = "Target timezone")]
    to_tz: String,
    #[clap(default_value = "2023-01-01 12:00:00", help = "Time in YYYY-MM-DD HH:mm:ss format")]
    time: String,
}

fn parse_datetime(datetime_str: &str) -> Result<NaiveDateTime, String> {
    NaiveDateTime::parse_from_str(datetime_str, "%Y-%m-%d %H:%M:%S").map_err(|_| "Invalid datetime format".to_string())
}

fn convert_timezone(from_tz: &str, to_tz: &str, datetime: &str) -> Result<(String, String), String> {
    // Parse the timezones
    let source_tz = Tz::from_str(from_tz).map_err(|_| format!("Invalid source timezone: {}", from_tz))?;
    let target_tz = Tz::from_str(to_tz).map_err(|_| format!("Invalid target timezone: {}", to_tz))?;

    // Parse the datetime
    let naive_datetime = parse_datetime(datetime)?;

    // Convert to source timezone
    let source_datetime: DateTime<Tz> = source_tz
        .from_local_datetime(&naive_datetime)
        .earliest()
        .ok_or("Invalid datetime")?;

    // Convert to target timezone
    let target_datetime = source_datetime.with_timezone(&target_tz);

    // Format the result
    let source_time_str = source_datetime.format("%Y-%m-%d %H:%M:%S").to_string();
    let target_time_str = target_datetime.format("%Y-%m-%d %H:%M:%S").to_string();

    Ok((source_time_str, target_time_str))
}

fn main() {
    let args = Cli::parse();

    match convert_timezone(&args.from_tz, &args.to_tz, &args.time) {
        Ok((source_time, target_time)) => {
            println!("From {} {}", source_time, args.from_tz);
            println!("To   {} {}", target_time, args.to_tz);
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
