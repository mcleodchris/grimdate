use super::*;
use chrono::TimeZone;

#[test]
fn test_format_date_2023() {
    let date = Utc.with_ymd_and_hms(2023, 10, 5, 15, 30, 0).unwrap();
    let formatted_date = format_date(date);
    assert_eq!(formatted_date, "0 760 023.M3");
}

#[test]
fn test_format_date_2025() {
    let date = Utc.with_ymd_and_hms(2025, 1, 23, 22, 30, 0).unwrap();
    let formatted_date = format_date(date);
    assert_eq!(formatted_date, "0 062 025.M3");
}

#[test]
fn test_format_date_1999() {
    let date = Utc.with_ymd_and_hms(1999, 1, 23, 22, 30, 0).unwrap();
    let formatted_date = format_date(date);
    assert_eq!(formatted_date, "0 062 999.M2");
}

#[test]
fn test_format_date_12() { // leap year
    let date = Utc.with_ymd_and_hms(12, 10, 5, 15, 30, 0).unwrap();
    let formatted_date = format_date(date);
    assert_eq!(formatted_date, "0 762 012.M1");
}