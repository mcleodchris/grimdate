use chrono::{DateTime, TimeZone, Utc, Local, Datelike, Timelike};
use clap::{Command, Arg};

fn format_date<T: TimeZone>(date: DateTime<T>, is_iss: bool, no_spaces: bool, no_time: bool) -> String {
    // Convert to UTC
    let utc_date = date.with_timezone(&Utc);
    
    // Get start of year
    let start_of_year = Utc.with_ymd_and_hms(utc_date.year(), 1, 1, 0, 0, 0).unwrap();
    
    // Calculate days since start of year
    let duration = utc_date.signed_duration_since(start_of_year);
    let days_since_start = duration.num_days();
    
    // Calculate determined hour
    let determined_hour = (days_since_start * 24) + utc_date.hour() as i64;
    
    // Calculate imperial fraction
    let imperial_fraction = (determined_hour as f64 * 0.11407955).floor() as u32;
    
    // Get year in current millennium
    let year = utc_date.year().abs() % 1000;
    
    // Calculate millennium
    let millennium = (utc_date.year().abs() / 1000) + 1;
    
    // Get local time
    let local_time = format!("{:02}:{:02}", date.hour(), date.minute());
    
    // Format the output
    let mut formatted_date = if no_spaces {
        format!(
            "{}{:03}{:03}.M{}",
            if is_iss { 1 } else { 0 },
            imperial_fraction,
            year,
            millennium
        )
    } else {
        format!(
            "{} {:03} {:03}.M{}",
            if is_iss { 1 } else { 0 },
            imperial_fraction,
            year,
            millennium
        )
    };
    
    if !no_time {
        formatted_date.push_str(&format!("//{} local", local_time));
    }
    
    formatted_date
}

#[cfg(test)]
mod tests;

fn main() {
    let matches = Command::new("GrimDate")
        .version("1.0")
        .author("Chris")
        .about("Formats dates in a specific way")
        .arg(
            Arg::new("no-spaces")
                .short('S')
                .long("no-spaces")
                .num_args(0)
                .help("Removes spaces from the output"),
        )
        .arg(
            Arg::new("is-iss")
                .short('i')
                .long("is-iss")
                .num_args(0)
                .help("Changes the leading 0 to 1"),
        )
        .arg(
            Arg::new("date")
                .short('d')
                .long("date")
                .value_name("DATE")
                .help("Specify the input date (YYYY-MM-DDTHH:MM:SS, timezone optional)")
                .value_parser(clap::value_parser!(String)),
        )
        .arg(
            Arg::new("no-time")
            .short('T')
            .long("no-time")
            .num_args(0)
            .help("Returns the date without the local time")
        )
        .get_matches();

    let no_spaces = matches.get_flag("no-spaces");
    let is_iss = matches.get_flag("is-iss");
    let no_time = matches.get_flag("no-time");
    let date = if let Some(date_str) = matches.get_one::<String>("date") {
        DateTime::parse_from_rfc3339(date_str).unwrap().with_timezone(&Utc)
    } else {
        Local::now().with_timezone(&Utc)
    };

    println!("{}", format_date(date, is_iss, no_spaces, no_time));
}