use chrono::{DateTime, Local, NaiveDateTime, TimeZone};
use thiserror::Error;

pub(crate) const MINUTE: i64 = 60;
pub(crate) const HOUR: i64 = MINUTE * 60;
pub(crate) const DAY: i64 = HOUR * 24;
pub(crate) const WEEK: i64 = DAY * 7;
pub(crate) const MONTH: i64 = DAY * 30;
pub(crate) const YEAR: i64 = DAY * 365;

#[derive(Error, Debug)]
pub enum TimeAgoError {
    #[error("Wrong datetime format !")]
    InvalidDateTimeFormat,
    #[error("Unexpected error happened !")]
    Unknown,
}
/// Converts a valid datetime to timestamp
///
/// # Warning
/// This function is desgined to only works for these date time formats :
///
///
/// - `%Y-%m-%d %H:%M:%S`: Sortable format
/// - `%Y/%m/%d %H:%M:%S`: Sortable format
/// - `%Y-%m-%dT%H:%M:%S%:z`: ISO 8601 with timezone offset
/// - `%Y-%m-%dT%H:%M:%S%.3f%:z`: ISO 8601 with milliseconds and timezone offset
/// - `%a, %d %b %Y %H:%M:%S %z`: RFC 2822 Format
///
///
///  timezone is set with the current timezone of the OS.
///
/// # Examples
///
/// ```
/// use rust_persian_tools::time_ago::convert_to_timestamp;
///
/// assert!(convert_to_timestamp("2023/12/30 12:21:13").is_ok());
/// assert!(convert_to_timestamp("2023/12/30 25:21:13").is_err());
/// ```
pub fn convert_to_timestamp(datetime: impl AsRef<str>) -> Result<i64, TimeAgoError> {
    let datetime = datetime.as_ref();
    let date_obj = get_date_time(datetime)?;

    Ok(date_obj.timestamp())
}

/// Converts datetime to Chrono `DateTime<Local>`
///
/// # Warning
/// This function is desgined to only works for these date time formats :
///
/// - `%Y-%m-%d %H:%M:%S`: Sortable format
/// - `%Y/%m/%d %H:%M:%S`: Sortable format
/// - `%Y-%m-%dT%H:%M:%S%:z`: ISO 8601 with timezone offset
/// - `%Y-%m-%dT%H:%M:%S%.3f%:z`: ISO 8601 with milliseconds and timezone offset
/// - `%a, %d %b %Y %H:%M:%S %z`: RFC 2822 Format
///
///  timezone is set with the current timezone of the OS.
///
/// # Examples
///
/// ```
/// use rust_persian_tools::time_ago::get_date_time;
///
/// assert!(get_date_time("2019/03/18 12:22:14").is_ok());
/// assert!(get_date_time("20192/03/18 12:22:14").is_err());
/// ```
pub fn get_date_time(datetime: impl AsRef<str>) -> Result<DateTime<Local>, TimeAgoError> {
    let datetime = datetime.as_ref();

    let formats = [
        "%Y-%m-%d %H:%M:%S",        // Sortable format
        "%Y/%m/%d %H:%M:%S",        // Sortable format
        "%Y-%m-%dT%H:%M:%S%:z",     // ISO 8601 with timezone offset
        "%Y-%m-%dT%H:%M:%S%.3f%:z", // ISO 8601 with milliseconds and timezone offset
        "%a, %d %b %Y %H:%M:%S %z", // RFC 2822 Format
    ];

    for format in formats {
        if let Ok(parsed) = NaiveDateTime::parse_from_str(datetime, format) {
            // Successfully parsed, convert to timestamp
            let datetime_with_timezone = Local.from_local_datetime(&parsed).earliest();
            return match datetime_with_timezone {
                Some(local_date_time) => Ok(local_date_time),
                None => Err(TimeAgoError::Unknown),
            };
        }
    }

    Err(TimeAgoError::InvalidDateTimeFormat)
}

/// Returns current timestamp
///
/// # Warning
///
///  timezone is set with the current timezone of the OS.
///
pub fn get_current_timestamp() -> i64 {
    let now = Local::now();
    now.timestamp()
}

/// Returns a string based on how much time is remaining or passed based on the givin datetime
///
/// # Warning
/// This function is desgined to only works for these date time formats :
///
/// - `%Y-%m-%d %H:%M:%S`: Sortable format
/// - `%Y/%m/%d %H:%M:%S`: Sortable format
/// - `%Y-%m-%dT%H:%M:%S%:z`: ISO 8601 with timezone offset
/// - `%Y-%m-%dT%H:%M:%S%.3f%:z`: ISO 8601 with milliseconds and timezone offset
/// - `%a, %d %b %Y %H:%M:%S %z`: RFC 2822 Format
///
///  timezone is set with the current timezone of the OS.
///
/// # Examples
///
/// ```
/// use rust_persian_tools::time_ago::time_ago;
/// use chrono::{Duration,Local};
///
/// let current_time = Local::now();
/// let ten_minutes_ago = current_time - Duration::minutes(10);
/// let formatted_time = ten_minutes_ago.format("%Y-%m-%d %H:%M:%S").to_string(); // create datetime string from 10 minutes ago
/// assert!(time_ago(Some(&formatted_time)).is_ok_and(|datetime| datetime == "10 دقیقه قبل"));
/// ```
pub fn time_ago(datetime: Option<impl AsRef<str>>) -> Result<String, TimeAgoError> {
    if datetime.is_none() {
        return Ok("اکنون".to_string());
    }

    let binding = datetime.unwrap();
    let datetime = binding.as_ref();

    let ts_now = get_current_timestamp();
    let ts = convert_to_timestamp(datetime)?;

    let elapsed = ts_now - ts;

    if (-1..=1).contains(&elapsed) {
        return Ok("اکنون".to_string());
    }

    let pre_or_next = if elapsed > 0 { "قبل" } else { "بعد" };

    let elapsed = elapsed.abs();

    if elapsed < MINUTE {
        let left = (elapsed as f64).round();
        Ok(format!("{} {} {}", left, "ثانیه", pre_or_next))
    } else if elapsed < HOUR {
        let left = ((elapsed / MINUTE) as f64).round();
        Ok(format!("{} {} {}", left, "دقیقه", pre_or_next))
    } else if elapsed < DAY {
        let left = ((elapsed / HOUR) as f64).round();
        Ok(format!("{} {} {}", left, "ساعت", pre_or_next))
    } else if elapsed < WEEK {
        let left = ((elapsed / DAY) as f64).round();
        Ok(format!("{} {} {} {}", "حدود", left, "روز", pre_or_next))
    } else if elapsed < MONTH {
        let left = ((elapsed / WEEK) as f64).round();
        Ok(format!("{} {} {} {}", "حدود", left, "هفته", pre_or_next))
    } else if elapsed < YEAR {
        let left = ((elapsed / MONTH) as f64).round();
        Ok(format!("{} {} {} {}", "حدود", left, "ماه", pre_or_next))
    } else {
        let left = ((elapsed / YEAR) as f64).round();
        Ok(format!("{} {} {} {}", "حدود", left, "سال", pre_or_next))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn test_time_ago_now() {
        let current_time = Local::now();
        // let ten_minutes_ago = current_time - Duration::minutes(10);
        let formatted_time = current_time.format("%Y-%m-%dT%H:%M:%S%:z").to_string();

        assert!(time_ago(Some(&formatted_time)).is_ok_and(|datetime| datetime == "اکنون"));
    }

    #[test]
    fn test_time_ago_10_min_ago() {
        let current_time = Local::now();
        let ten_minutes_ago = current_time - Duration::minutes(10);
        let formatted_time = ten_minutes_ago.format("%Y-%m-%d %H:%M:%S").to_string();

        assert!(time_ago(Some(&formatted_time)).is_ok_and(|datetime| datetime == "10 دقیقه قبل"));
    }

    #[test]
    fn test_time_ago_next_2_weeks() {
        let current_time = Local::now();
        let ten_minutes_ago = current_time + Duration::weeks(2);
        let formatted_time = ten_minutes_ago
            .format("%a, %d %b %Y %H:%M:%S %z")
            .to_string();

        assert!(time_ago(Some(&formatted_time)).is_ok_and(|datetime| datetime == "حدود 2 هفته بعد"));
    }

    #[test]
    fn test_time_ago_next_3_months() {
        let current_time = Local::now();
        let ten_minutes_ago = current_time + Duration::days(31 * 3);
        let formatted_time = ten_minutes_ago
            .format("%Y-%m-%dT%H:%M:%S%.3f%:z")
            .to_string();

        assert!(time_ago(Some(&formatted_time)).is_ok_and(|datetime| datetime == "حدود 3 ماه بعد"));
    }

    #[test]
    fn test_check_valid_date_time() {
        assert!(get_date_time("2019/03/18 12:22:14").is_ok());
        assert!(get_date_time("20192/03/18 12:22:14").is_err());
    }
}
