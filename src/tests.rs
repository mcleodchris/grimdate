use super::*;
use chrono::{FixedOffset, TimeZone};

#[test]
fn test_format_date_2023() {
    let date = Utc.with_ymd_and_hms(2023, 10, 5, 15, 30, 0).unwrap();
    let formatted_date = format_date(date, false, false, false);
    assert_eq!(formatted_date, "0 760 023.M3//15:30 local");
}

#[test]
fn test_format_date_2025() {
    let date = Utc.with_ymd_and_hms(2025, 1, 23, 22, 30, 0).unwrap();
    let formatted_date = format_date(date, false, false, false);
    assert_eq!(formatted_date, "0 062 025.M3//22:30 local");
}

#[test]
fn test_format_date_1999() {
    let date = Utc.with_ymd_and_hms(1999, 1, 23, 22, 30, 0).unwrap();
    let formatted_date = format_date(date, false, false, false);
    assert_eq!(formatted_date, "0 062 999.M2//22:30 local");
}

#[test]
fn test_format_date_12() { // leap year
    let date = Utc.with_ymd_and_hms(12, 10, 5, 15, 30, 0).unwrap();
    let formatted_date = format_date(date, false, false, false);
    assert_eq!(formatted_date, "0 762 012.M1//15:30 local");
}
#[test]
fn test_format_date_2023_on_iss() {
    let date = Utc.with_ymd_and_hms(2023, 10, 5, 15, 30, 45).unwrap();
    let formatted_date = format_date(date, true, false, false);
    assert_eq!(formatted_date, "1 760 023.M3//15:30 local");
}

#[test]
fn test_format_date_2025_on_iss() {
    let date = Utc.with_ymd_and_hms(2025, 1, 23, 22, 30, 15).unwrap();
    let formatted_date = format_date(date, true, false, false);
    assert_eq!(formatted_date, "1 062 025.M3//22:30 local");
}

#[test]
fn test_format_date_1999_on_iss() {
    let date = Utc.with_ymd_and_hms(1999, 1, 23, 22, 30, 59).unwrap();
    let formatted_date = format_date(date, true, false, false);
    assert_eq!(formatted_date, "1 062 999.M2//22:30 local");
}

#[test]
fn test_format_date_12_on_iss() { // leap year
    let date = Utc.with_ymd_and_hms(12, 10, 5, 15, 30, 30).unwrap();
    let formatted_date = format_date(date, true, false, false);
    assert_eq!(formatted_date, "1 762 012.M1//15:30 local");
}

#[test]
fn test_format_date_2023_no_spaces() {
    let date = Utc.with_ymd_and_hms(2023, 10, 5, 15, 30, 0).unwrap();
    let formatted_date = format_date(date, false, true, false);
    assert_eq!(formatted_date, "0760023.M3//15:30 local");
}

#[test]
fn test_format_date_2025_no_spaces() {
    let date = Utc.with_ymd_and_hms(2025, 1, 23, 22, 30, 0).unwrap();
    let formatted_date = format_date(date, false, true, false);
    assert_eq!(formatted_date, "0062025.M3//22:30 local");
}

#[test]
fn test_format_date_1999_no_spaces() {
    let date = Utc.with_ymd_and_hms(1999, 1, 23, 22, 30, 0).unwrap();
    let formatted_date = format_date(date, false, true, false);
    assert_eq!(formatted_date, "0062999.M2//22:30 local");
}

#[test]
fn test_format_date_12_no_spaces() { // leap year
    let date = Utc.with_ymd_and_hms(12, 10, 5, 15, 30, 0).unwrap();
    let formatted_date = format_date(date, false, true, false);
    assert_eq!(formatted_date, "0762012.M1//15:30 local");
}

#[test]
fn test_format_date_2023_non_utc() {
    let date = FixedOffset::east_opt(5 * 3600).unwrap().with_ymd_and_hms(2023, 10, 5, 15, 30, 0).unwrap();
    let formatted_date = format_date(date, false, false, false);
    assert_eq!(formatted_date, "0 759 023.M3//15:30 local");
}

// add tests for no_time
#[test]
fn test_format_date_2023_no_time() {
    let date = Utc.with_ymd_and_hms(2023, 10, 5, 15, 30, 0).unwrap();
    let formatted_date = format_date(date, false, false, true);
    assert_eq!(formatted_date, "0 760 023.M3");
}