use chrono::{DateTime, Local, NaiveDateTime, TimeZone};
use thiserror::Error;

pub(crate) const MINUTE: i64 = 60;
pub(crate) const HOUR: i64 = MINUTE * 60;
pub(crate) const DAY: i64 = HOUR * 24;
pub(crate) const MONTH: i64 = DAY * 30;
pub(crate) const YEAR: i64 = DAY * 365;

#[derive(Error, Debug)]
pub enum TimeAgoError {
    #[error("Wrong datetime format !")]
    InvalidDateTimeFormat,
    #[error("Unexpected error happened !")]
    Unknown,
}

/// The [TimeDiff] stuct has two methods , `short_form()` & `long_form()` \
/// the `short_form()` returns a short desciption about time diffrence\
/// - 5 دقیقه قبل
/// - حدود 2 هفته بعد
///
/// the `long_form()` returns a long and exact desciption about time diffrence\
/// - 6 سال و 6 ماه و 10 روز و 12 دقیقه و 37 ثانیه بعد
///
#[derive(Debug, PartialEq)]
pub struct TimeDiff {
    pub years: u32,
    pub months: u8,
    pub days: u8,
    pub hours: u8,
    pub minutes: u8,
    pub seconds: u8,
    pub is_remaining_time: bool,
}

impl TimeDiff {
    pub fn long_form(&self) -> String {
        let mut periods: Vec<String> = Vec::new();

        let pre_or_next = pre_or_next(self.is_remaining_time);

        if self.years > 0 {
            periods.push(format!("{} سال", self.years))
        }
        if self.months > 0 {
            periods.push(format!("{} ماه", self.months))
        }
        if self.days > 0 {
            periods.push(format!("{} روز", self.days))
        }
        if self.hours > 0 {
            periods.push(format!("{} ساعت", self.hours))
        }
        if self.minutes > 0 {
            periods.push(format!("{} دقیقه", self.minutes))
        }
        if self.seconds > 0 {
            periods.push(format!("{} ثانیه", self.seconds))
        }

        format!("{} {}", periods.join(" و "), pre_or_next)
    }

    pub fn short_form(&self) -> String {
        let pre_or_next = pre_or_next(self.is_remaining_time);

        if self.years != 0 {
            format!("{} {} {} {}", "حدود", &self.years, "سال", pre_or_next)
        } else if self.months != 0 {
            format!("{} {} {} {}", "حدود", &self.months, "ماه", pre_or_next)
        } else if self.days > 7 {
            format!("{} {} {} {}", "حدود", &self.days / 7, "هفته", pre_or_next)
        } else if self.days != 0 {
            format!("{} {} {} {}", "حدود", &self.days, "روز", pre_or_next)
        } else if self.hours != 0 {
            format!("{} {} {}", &self.hours, "ساعت", pre_or_next)
        } else if self.minutes != 0 {
            format!("{} {} {}", &self.minutes, "دقیقه", pre_or_next)
        } else if self.seconds != 0 {
            format!("{} {} {}", &self.seconds, "ثانیه", pre_or_next)
        } else {
            "اکنون".to_string()
        }
    }
}

fn pre_or_next(is_remaining_time: bool) -> String {
    if is_remaining_time {
        "بعد".to_owned()
    } else {
        "قبل".to_owned()
    }
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
/// use rust_persian_tools::time_diff::convert_to_timestamp;
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
/// use rust_persian_tools::time_diff::get_date_time;
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

/// Returns a [TimeDiff] stuct based on how much time is remaining or passed based on the givin datetime\
/// The [TimeDiff] stuct has two methods , `short_form()` & `long_form()` \
/// the `short_form()` returns a short desciption about time diffrence\
/// - 5 دقیقه قبل
/// - حدود 2 هفته بعد
///
/// the `long_form()` returns a long and exact desciption about time diffrence\
/// - 6 سال و 6 ماه و 10 روز و 12 دقیقه و 37 ثانیه بعد
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
/// use rust_persian_tools::time_diff::{TimeDiff , time_diff};
/// use chrono::{Duration,Local};
///
/// let current_time = Local::now();
/// let due_date = current_time
/// + Duration::weeks(320)
/// + Duration::hours(7)
/// + Duration::minutes(13)
/// + Duration::seconds(37);
/// let formatted_time = due_date.format("%Y-%m-%d %H:%M:%S").to_string();
/// assert_eq!(
/// time_diff(formatted_time).unwrap(),
///   TimeDiff {
///       years: 6,
///       months: 1,
///       days: 20,
///       hours: 7,
///       minutes: 13,
///       seconds: 37,
///       is_remaining_time: true,
///   }
/// );
///
/// // Example with short_form()
/// let current_time = Local::now();
/// let ten_minutes_ago = current_time - Duration::minutes(10);
/// let formatted_time = ten_minutes_ago.format("%Y-%m-%d %H:%M:%S").to_string(); // create datetime string from 10 minutes ago
/// assert!(time_diff(formatted_time).is_ok_and(|datetime| datetime.short_form() == "10 دقیقه قبل"));
/// ```
pub fn time_diff(datetime: impl AsRef<str>) -> Result<TimeDiff, TimeAgoError> {
    let datetime = datetime.as_ref();

    let ts_now = get_current_timestamp();
    let ts = convert_to_timestamp(datetime)?;

    let timestamp_diff = ts - ts_now;

    let is_remaining_time = timestamp_diff > 0;

    let mut timestamp_diff = timestamp_diff.abs();

    let years: u32 = (timestamp_diff / YEAR) as u32;
    timestamp_diff %= YEAR;

    let months: u8 = ((timestamp_diff / MONTH) % MONTH) as u8;
    timestamp_diff %= MONTH;

    let days: u8 = ((timestamp_diff / DAY) % DAY) as u8;
    timestamp_diff %= DAY;

    let hours: u8 = ((timestamp_diff / HOUR) % HOUR) as u8;
    timestamp_diff %= HOUR;

    let minutes: u8 = ((timestamp_diff / MINUTE) % MINUTE) as u8;
    timestamp_diff %= MINUTE;

    let seconds: u8 = timestamp_diff as u8;

    Ok(TimeDiff {
        years,
        months,
        days,
        hours,
        minutes,
        seconds,
        is_remaining_time,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn test_time_diff_now() {
        let current_time = Local::now();
        // let ten_minutes_ago = current_time - Duration::minutes(10);
        let formatted_time = current_time.format("%Y-%m-%dT%H:%M:%S%:z").to_string();

        assert!(time_diff(&formatted_time).is_ok_and(|datetime| datetime.short_form() == "اکنون"));
    }

    #[test]
    fn test_time_diff_10_min_ago() {
        let current_time = Local::now();
        let ten_minutes_ago = current_time - Duration::minutes(10);
        let formatted_time = ten_minutes_ago.format("%Y-%m-%d %H:%M:%S").to_string();

        // dbg!(time_diff(&formatted_time))
        assert!(time_diff(&formatted_time)
            .is_ok_and(|datetime| datetime.short_form() == "10 دقیقه قبل"));
    }

    #[test]
    fn test_time_diff_next_2_weeks() {
        let current_time = Local::now();
        let ten_minutes_ago = current_time + Duration::weeks(2);
        let formatted_time = ten_minutes_ago
            .format("%a, %d %b %Y %H:%M:%S %z")
            .to_string();

        assert!(time_diff(&formatted_time)
            .is_ok_and(|datetime| datetime.short_form() == "حدود 2 هفته بعد"));
    }

    #[test]
    fn test_time_diff_next_3_months() {
        let current_time = Local::now();
        let ten_minutes_ago = current_time + Duration::days(31 * 3);
        let formatted_time = ten_minutes_ago
            .format("%Y-%m-%dT%H:%M:%S%.3f%:z")
            .to_string();

        assert!(time_diff(&formatted_time)
            .is_ok_and(|datetime| datetime.short_form() == "حدود 3 ماه بعد"));
    }

    #[test]
    fn test_time_diff_as_struct() {
        let current_time = Local::now();
        let due_date = current_time
            + Duration::weeks(320)
            + Duration::hours(7)
            + Duration::minutes(13)
            + Duration::seconds(37);
        let formatted_time = due_date.format("%Y-%m-%d %H:%M:%S").to_string();

        assert_eq!(
            time_diff(formatted_time).unwrap(),
            TimeDiff {
                years: 6,
                months: 1,
                days: 20,
                hours: 7,
                minutes: 13,
                seconds: 37,
                is_remaining_time: true,
            }
        );
    }

    #[test]
    fn test_time_diff_as_long_form() {
        let current_time = Local::now();
        let due_date =
            current_time + Duration::weeks(340) + Duration::minutes(12) + Duration::seconds(37);
        let formatted_time = due_date.format("%Y-%m-%d %H:%M:%S").to_string();

        assert_eq!(
            time_diff(formatted_time).unwrap().long_form(),
            String::from("6 سال و 6 ماه و 10 روز و 12 دقیقه و 37 ثانیه بعد")
        );
    }

    #[test]
    fn test_check_valid_date_time() {
        assert!(get_date_time("2019/03/18 12:22:14").is_ok());
        assert!(get_date_time("20192/03/18 12:22:14").is_err());
    }
}
