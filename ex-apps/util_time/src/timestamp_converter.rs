use chrono::{TimeZone, Utc};
use chrono_tz::Tz;
use clap::Parser;
use std::str::FromStr; // Import the FromStr trait

#[derive(Parser)]
struct Cli {
    #[clap(default_value = "0", help = "Timestamp in seconds since the Unix epoch")]
    timestamp: String,
    #[clap(default_value = "Australia/Sydney", help = "Target timezone")]
    to_tz: String,
}

fn convert_timestamp(timestamp: i64, to_tz: &str) -> Result<String, String> {
    // Parse the target timezone
    let target_tz = Tz::from_str(to_tz).map_err(|_| format!("Invalid target timezone: {}", to_tz))?;

    // Convert the timestamp to a DateTime<Utc>
    let utc_datetime = Utc.timestamp_opt(timestamp, 0).single().ok_or("Invalid timestamp")?;

    // Convert to target timezone
    let target_datetime = utc_datetime.with_timezone(&target_tz);

    // Format the result
    Ok(target_datetime.format("%Y-%m-%d %H:%M:%S%.3f").to_string())
}

fn main() {
    let args = Cli::parse();

    let timestamp: i64 = args.timestamp.parse().unwrap_or(0);

    match convert_timestamp(timestamp, &args.to_tz) {
        Ok(result) => println!("{}", result),
        Err(e) => eprintln!("Error: {}", e),
    }
}
