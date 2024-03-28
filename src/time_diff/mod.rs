use chrono::{DateTime, Local, NaiveDateTime, TimeZone};
use thiserror::Error;

use crate::digits::{DigitsEn2Ar, DigitsEn2Fa};

pub(crate) const MINUTE: i64 = 60;
pub(crate) const HOUR: i64 = MINUTE * 60;
pub(crate) const DAY: i64 = HOUR * 24;
pub(crate) const MONTH: i64 = DAY * 30;
pub(crate) const YEAR: i64 = DAY * 365;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Error, Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum TimeAgoError {
    #[error("Wrong datetime format !")]
    InvalidDateTimeFormat,
    #[error("Unexpected error happened !")]
    Unknown,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum Timestamp {
    String(String),
    Integer(i64),
}

impl From<String> for Timestamp {
    fn from(datetime_str: String) -> Self {
        Timestamp::String(datetime_str)
    }
}

impl From<&str> for Timestamp {
    fn from(datetime_str: &str) -> Self {
        Timestamp::String(datetime_str.to_string())
    }
}

impl From<i64> for Timestamp {
    fn from(timestamp: i64) -> Self {
        Timestamp::Integer(timestamp)
    }
}

/// The [TimeDiff] stuct has two main methods: `short_form()` & `long_form()` \
/// the `short_form()` returns a short desciption about time diffrence\
/// - 5 دقیقه قبل
/// - حدود 2 هفته بعد
///
/// the `long_form()` returns a long and exact desciption about time diffrence\
/// - 6 سال و 6 ماه و 10 روز و 12 دقیقه و 37 ثانیه بعد
///
/// also there are more methods to return long_from and short with arabic or persian digits
/// - short_form_fa_digits()
/// - short_form_ar_digits()
/// - long_form_fa_digits()
/// - long_form_ar_digits()
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
pub struct TimeDiff {
    pub years: u32,
    pub months: u8,
    pub days: u8,
    pub hours: u8,
    pub minutes: u8,
    pub seconds: u8,
    pub is_future: bool,
}

impl TimeDiff {
    pub fn long_form(&self) -> String {
        let mut periods: Vec<String> = Vec::new();

        let pre_or_next = self.pre_or_next();

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
        let pre_or_next = self.pre_or_next();

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

    pub fn short_form_fa_digits(&self) -> String {
        self.short_form().digits_en_to_fa()
    }

    pub fn long_form_fa_digits(&self) -> String {
        self.long_form().digits_en_to_fa()
    }

    pub fn short_form_ar_digits(&self) -> String {
        self.short_form().digits_en_to_ar()
    }

    pub fn long_form_ar_digits(&self) -> String {
        self.long_form().digits_en_to_ar()
    }

    pub fn pre_or_next(&self) -> String {
        if self.is_future {
            "بعد".to_owned()
        } else {
            "قبل".to_owned()
        }
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

/// datetime argument can be a integer as timestamp or a string as datetime
/// Returns a [TimeDiff] stuct based on how much time is remaining or passed based on the givin datetime\
/// The [TimeDiff] stuct has two methods , `short_form()` & `long_form()` \
///
/// the `short_form()` returns a short desciption about time diffrence\
/// - 5 دقیقه قبل
/// - حدود 2 هفته بعد
///
/// the `long_form()` returns a long and exact desciption about time diffrence\
/// - 6 سال و 6 ماه و 10 روز و 12 دقیقه و 37 ثانیه بعد
///
/// also there are some other methords like `short_form_fa_digits()` or `short_form_ar_digits()` that is the same as `short_form()` but with farsi or arabic digits
///
/// # Warning
/// This function is desgined to only works for these date time formats if you send datetime argument as datetime string :
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
/// use rust_persian_tools::time_diff::{TimeDiff , time_diff_now};
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
/// time_diff_now(formatted_time).unwrap(),
///   TimeDiff {
///       years: 6,
///       months: 1,
///       days: 20,
///       hours: 7,
///       minutes: 13,
///       seconds: 37,
///       is_future: true,
///   }
/// );
///
/// // Example with short_form()
/// let current_time = Local::now();
/// let ten_minutes_ago = current_time - Duration::minutes(10);
/// let formatted_time = ten_minutes_ago.format("%Y-%m-%d %H:%M:%S").to_string(); // create datetime string from 10 minutes ago
/// assert!(time_diff_now(formatted_time).is_ok_and(|datetime| datetime.short_form() == "10 دقیقه قبل"));
/// ```
pub fn time_diff_now(datetime: impl Into<Timestamp>) -> Result<TimeDiff, TimeAgoError> {
    let ts_now = get_current_timestamp();
    let ts = match datetime.into() {
        Timestamp::String(datetime_str) => convert_to_timestamp(datetime_str)?,
        Timestamp::Integer(timestamp) => timestamp,
    };

    let timestamp_diff = ts - ts_now;

    Ok(get_time_diff(timestamp_diff))
}

/// start & end arguments can be a integer as timestamp or a string as datetime
/// Returns a [TimeDiff] stuct based on how much time is remaining or passed based on the diffrence between two datetime\
/// The [TimeDiff] stuct has two main methods , `short_form()` & `long_form()` \
/// the `short_form()` returns a short desciption about time diffrence\
/// - 5 دقیقه قبل
/// - حدود 2 هفته بعد
///
/// the `long_form()` returns a long and exact desciption about time diffrence\
/// - 6 سال و 6 ماه و 10 روز و 12 دقیقه و 37 ثانیه بعد
///
/// also there are some other methords like `short_form_fa_digits()` or `short_form_ar_digits()` that is the same as `short_form()` but with farsi or arabic digits
///
/// # Warning
/// This function is desgined to only works for these datetime formats if you send start or end as datetime string:
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
/// use rust_persian_tools::time_diff::{TimeDiff , time_diff_between};
/// use chrono::{Duration,Local};
///
/// let current_time = Local::now();
/// let start = current_time
///     + Duration::weeks(320)
///     + Duration::hours(7)
///     + Duration::minutes(13)
///     + Duration::seconds(37);
/// let end = (current_time + Duration::weeks(150) + Duration::hours(4)).timestamp();
/// let formatted_time = start.format("%Y-%m-%d %H:%M:%S").to_string();
/// assert_eq!(
///     time_diff_between(formatted_time, end).unwrap(),
///     TimeDiff {
///         years: 3,
///         months: 3,
///         days: 5,
///         hours: 3,
///         minutes: 13,
///         seconds: 37,
///         is_future: false,
///     }
/// );
///
/// // Example with long_form() with persian digits
//  let current_time = Local::now();
//  let start = current_time
//     + Duration::weeks(320)
//     + Duration::hours(7)
//     + Duration::minutes(13)
//     + Duration::seconds(37);
//
// let end = (current_time + Duration::weeks(150) + Duration::hours(4)).timestamp();
//
// let formatted_time = start.format("%Y-%m-%d %H:%M:%S").to_string();
// assert_eq!(
//     time_diff_between(formatted_time, end)
//         .unwrap()
//         .long_form_fa_digits(),
//     "۳ سال و ۳ ماه و ۵ روز و ۳ ساعت و ۱۳ دقیقه و ۳۷ ثانیه قبل"
// );
/// ```
pub fn time_diff_between(
    start: impl Into<Timestamp>,
    end: impl Into<Timestamp>,
) -> Result<TimeDiff, TimeAgoError> {
    let ts_start = match start.into() {
        Timestamp::String(datetime_str) => convert_to_timestamp(datetime_str)?,
        Timestamp::Integer(timestamp) => timestamp,
    };

    let ts_end = match end.into() {
        Timestamp::String(datetime_str) => convert_to_timestamp(datetime_str)?,
        Timestamp::Integer(timestamp) => timestamp,
    };

    let timestamp_diff = ts_end - ts_start;

    Ok(get_time_diff(timestamp_diff))
}

fn get_time_diff(timestamp_diff: i64) -> TimeDiff {
    let is_future = timestamp_diff > 0;

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

    TimeDiff {
        years,
        months,
        days,
        hours,
        minutes,
        seconds,
        is_future,
    }
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

        assert!(
            time_diff_now(formatted_time).is_ok_and(|datetime| datetime.short_form() == "اکنون")
        );
    }

    #[test]
    fn test_time_diff_10_min_ago() {
        let current_time = Local::now();
        let ten_minutes_ago = current_time - Duration::try_minutes(10).unwrap();
        let formatted_time = ten_minutes_ago.format("%Y-%m-%d %H:%M:%S").to_string();

        // dbg!(time_diff(&formatted_time))
        assert!(time_diff_now(formatted_time)
            .is_ok_and(|datetime| datetime.short_form() == "10 دقیقه قبل"));
    }

    #[test]
    fn test_time_diff_between_to_datetime() {
        let current_time = Local::now();
        let start = current_time
            + Duration::try_weeks(320).unwrap()
            + Duration::try_hours(7).unwrap()
            + Duration::try_minutes(13).unwrap()
            + Duration::try_seconds(37).unwrap();

        let end =
            (current_time + Duration::try_weeks(150).unwrap() + Duration::try_hours(4).unwrap())
                .timestamp();

        let formatted_time = start.format("%Y-%m-%d %H:%M:%S").to_string();
        assert_eq!(
            time_diff_between(formatted_time, end).unwrap(),
            TimeDiff {
                years: 3,
                months: 3,
                days: 5,
                hours: 3,
                minutes: 13,
                seconds: 37,
                is_future: false,
            }
        );
    }

    #[test]
    fn test_time_diff_between_to_datetime_with_long_format_persian_digits() {
        let current_time = Local::now();
        let start = current_time
            + Duration::try_weeks(320).unwrap()
            + Duration::try_hours(7).unwrap()
            + Duration::try_minutes(13).unwrap()
            + Duration::try_seconds(37).unwrap();

        let end =
            (current_time + Duration::try_weeks(150).unwrap() + Duration::try_hours(4).unwrap())
                .timestamp();

        let formatted_time = start.format("%Y-%m-%d %H:%M:%S").to_string();
        assert_eq!(
            time_diff_between(formatted_time, end)
                .unwrap()
                .long_form_fa_digits(),
            "۳ سال و ۳ ماه و ۵ روز و ۳ ساعت و ۱۳ دقیقه و ۳۷ ثانیه قبل"
        );
    }

    #[test]
    fn test_time_diff_next_2_weeks() {
        let current_time = Local::now();
        let ten_minutes_ago = current_time + Duration::try_weeks(2).unwrap();
        let formatted_time = ten_minutes_ago
            .format("%a, %d %b %Y %H:%M:%S %z")
            .to_string();

        assert!(time_diff_now(formatted_time)
            .is_ok_and(|datetime| datetime.short_form() == "حدود 2 هفته بعد"));
    }

    #[test]
    fn test_time_diff_next_3_months() {
        let current_time = Local::now();
        let ten_minutes_ago = current_time + Duration::try_days(31 * 3).unwrap();
        let formatted_time = ten_minutes_ago
            .format("%Y-%m-%dT%H:%M:%S%.3f%:z")
            .to_string();

        assert!(time_diff_now(formatted_time)
            .is_ok_and(|datetime| datetime.short_form() == "حدود 3 ماه بعد"));
    }

    #[test]
    fn test_time_diff_as_struct() {
        let current_time = Local::now();
        let due_date = current_time
            + Duration::try_weeks(320).unwrap()
            + Duration::try_hours(7).unwrap()
            + Duration::try_minutes(13).unwrap()
            + Duration::try_seconds(37).unwrap();
        let formatted_time = due_date.format("%Y-%m-%d %H:%M:%S").to_string();

        assert_eq!(
            time_diff_now(formatted_time).unwrap(),
            TimeDiff {
                years: 6,
                months: 1,
                days: 20,
                hours: 7,
                minutes: 13,
                seconds: 37,
                is_future: true,
            }
        );
    }

    #[test]
    fn test_time_diff_as_long_form() {
        let current_time = Local::now();
        let due_date = current_time
            + Duration::try_weeks(340).unwrap()
            + Duration::try_minutes(12).unwrap()
            + Duration::try_seconds(37).unwrap();
        let formatted_time = due_date.format("%Y-%m-%d %H:%M:%S").to_string();

        assert_eq!(
            time_diff_now(formatted_time).unwrap().long_form(),
            String::from("6 سال و 6 ماه و 10 روز و 12 دقیقه و 37 ثانیه بعد")
        );
    }

    #[test]
    fn test_check_valid_date_time() {
        assert!(get_date_time("2019/03/18 12:22:14").is_ok());
        assert!(get_date_time("20192/03/18 12:22:14").is_err());
    }
}
