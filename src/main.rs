use chrono::{DateTime, TimeZone, Utc, Datelike, Timelike};

fn format_date<T: TimeZone>(date: DateTime<T>) -> String {
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
    
    format!("0 {:03} {:03}.M{}", imperial_fraction, year, millennium)
}

#[cfg(test)]
mod tests;

// Example usage:
fn main() {
    let date = Utc::now();
    println!("{}", format_date(date));
}