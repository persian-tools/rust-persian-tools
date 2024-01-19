use std::fmt;

use crate::time_ago::{
    convert_to_timestamp, get_current_timestamp, TimeAgoError, DAY, HOUR, MINUTE, MONTH, YEAR,
};

#[derive(Debug, PartialEq)]
pub struct RemainingTime {
    pub years: u32,
    pub months: u8,
    pub days: u8,
    pub hours: u8,
    pub minutes: u8,
    pub seconds: u8,
    pub is_finished: bool,
}

impl fmt::Display for RemainingTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Format the struct fields as needed

        let mut periods: Vec<String> = Vec::new();

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

        write!(f, "{}", periods.join(" و "))
    }
}

/// returns [RemainingTime] as result if the datetime has a valid format
///
/// in [RemainingTime] you can use its field like ```remaining_time.hours``` or ```remaining_time.days```
/// also Display trais is implmented for [RemainingTime] so if you could try try ```println("{}" , remaining_time)``` and a string like : ```۱ سال و ۱ ماه و ۲ روز و ۳ ساعت و ۵ دقیقه و ۸ ثانیه```
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
/// ```
/// use rust_persian_tools::remaining_time::{remaining_time , RemainingTime};
/// use chrono::{Duration, Local};
///
/// let current_time = Local::now();
/// let due_date = current_time
///     + Duration::weeks(320)
///     + Duration::hours(7)
///     + Duration::minutes(13)
///     + Duration::seconds(37);
/// let formatted_time = due_date.format("%Y-%m-%d %H:%M:%S").to_string();
///
/// assert_eq!(
///     remaining_time(&formatted_time).unwrap(),
///     RemainingTime {
///         years: 6,
///         months: 1,
///         days: 20,
///         hours: 7,
///         minutes: 13,
///         seconds: 37,
///         is_finished: false,
///     }
/// );
///
/// assert_eq!(
///     format!("{}", remaining_time(&formatted_time).unwrap()),
///     String::from("6 سال و 1 ماه و 20 روز و 7 ساعت و 13 دقیقه و 37 ثانیه")
/// );
///
///
/// ```
pub fn remaining_time(datetime: impl AsRef<str>) -> Result<RemainingTime, TimeAgoError> {
    let datetime = datetime.as_ref();

    let due_date = convert_to_timestamp(datetime)?;
    let now = get_current_timestamp();

    let mut remaining_timestamp = due_date - now;

    if remaining_timestamp <= 0 {
        // if its due
        return Ok(RemainingTime {
            years: 0,
            months: 0,
            days: 0,
            hours: 0,
            minutes: 0,
            seconds: 0,
            is_finished: true,
        });
    }

    let years: u32 = (remaining_timestamp / YEAR) as u32;
    remaining_timestamp %= YEAR;

    let months: u8 = ((remaining_timestamp / MONTH) % MONTH) as u8;
    remaining_timestamp %= MONTH;

    let days: u8 = ((remaining_timestamp / DAY) % DAY) as u8;
    remaining_timestamp %= DAY;

    let hours: u8 = ((remaining_timestamp / HOUR) % HOUR) as u8;
    remaining_timestamp %= HOUR;

    let minutes: u8 = ((remaining_timestamp / MINUTE) % MINUTE) as u8;
    remaining_timestamp %= MINUTE;

    let seconds: u8 = remaining_timestamp as u8;

    Ok(RemainingTime {
        years,
        months,
        days,
        hours,
        minutes,
        seconds,
        is_finished: false,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Duration, Local};

    #[test]
    fn remaining_time_test() {
        let current_time = Local::now();
        let due_date = current_time
            + Duration::weeks(320)
            + Duration::hours(7)
            + Duration::minutes(13)
            + Duration::seconds(37);
        let formatted_time = due_date.format("%Y-%m-%d %H:%M:%S").to_string();

        assert_eq!(
            remaining_time(formatted_time).unwrap(),
            RemainingTime {
                years: 6,
                months: 1,
                days: 20,
                hours: 7,
                minutes: 13,
                seconds: 37,
                is_finished: false,
            }
        );
    }

    #[test]
    fn remaining_time_as_string_test() {
        let current_time = Local::now();
        let due_date =
            current_time + Duration::weeks(340) + Duration::minutes(12) + Duration::seconds(37);
        let formatted_time = due_date.format("%Y-%m-%d %H:%M:%S").to_string();

        assert_eq!(
            format!("{}", remaining_time(formatted_time).unwrap()),
            String::from("6 سال و 6 ماه و 10 روز و 12 دقیقه و 37 ثانیه")
        );
    }

    #[test]
    fn remaining_time_fail_test() {
        assert!(remaining_time("123:12312").is_err());
    }
}
