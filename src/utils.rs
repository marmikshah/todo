//! Utility functions for date parsing and formatting.

use chrono::prelude::*;

/// Parses a date string in YYYYMMDD format and returns a DateTime.
///
/// The returned DateTime is set to 23:59:59 (end of day) for the given date.
/// Only future dates or today's date are accepted.
///
/// # Arguments
///
/// * `date` - Date string in YYYYMMDD format (e.g., "20241225")
///
/// # Returns
///
/// * `Ok(DateTime<Local>)` - Successfully parsed date set to end of day
/// * `Err(String)` - Error message describing what went wrong
///
/// # Errors
///
/// This function will return an error if:
/// * The date string is not in YYYYMMDD format
/// * The date is in the past (before today)
/// * The date string contains invalid date values
pub fn parse_date(date: &str) -> Result<DateTime<Local>, String> {
    match NaiveDate::parse_from_str(date, "%Y%m%d") {
        Ok(naive_date) => {
            let value = naive_date.and_hms_opt(23, 59, 59).unwrap();
            let now = Local::now().naive_local();
            if value.date() < now.date() {
                return Err("date is in the past".to_string());
            }
            Ok(value.and_local_timezone(Local).unwrap())
        }
        Err(_) => Err("Invalid date format. Please use YYYYMMDD format".to_string()),
    }
}

/// Returns a DateTime representing the end of today (23:59:59).
///
/// # Returns
///
/// A `DateTime<Local>` set to 23:59:59 of the current day.
///
/// # Safety
///
/// This function uses `unwrap()` internally, but it's safe because:
/// * `NaiveTime::from_hms_opt(23, 59, 59)` will never fail (valid time)
/// * `Local::now().with_time()` will never fail (valid timezone conversion)
pub fn get_end_of_today() -> DateTime<Local> {
    Local::now()
        .with_time(NaiveTime::from_hms_opt(23, 59, 59).unwrap())
        .unwrap()
}
