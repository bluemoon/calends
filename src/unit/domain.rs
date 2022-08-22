use std::fmt::Display;

use chrono::NaiveDate;

use crate::{Interval, RelativeDuration};

/// A unit in time
///
/// # Rationale for this over interval
///
/// The calendar unit (name tbd) is more useful when you have fixed frames of time than an
/// interval. An interval is flexible and can represent durations of time such as one quarter and
/// one day.
///
/// An interval could be converted into a fixed chunk of time but it would potentially require
/// searching and may be a little confusing. It may also be advantageous for the consumer of the
/// API to do things like iterate by actual quarters.
///
#[derive(Debug, PartialEq, Eq)]
pub enum CalendarUnit {
    Year(i32),
    Quarter(i32, u8),
    Half(i32, u8),
    Month(i32, u8),
    Week(i32, u8),
}

impl CalendarUnit {
    pub fn into_interval(&self) -> Interval {
        match self {
            CalendarUnit::Year(year) => {
                Interval::from_start(NaiveDate::from_yo(*year, 1), RelativeDuration::months(12))
            }
            CalendarUnit::Quarter(year, quarter) => Interval::from_start(
                NaiveDate::from_ymd(*year, (*quarter * 3 - 2).try_into().unwrap(), 1),
                RelativeDuration::months(3),
            ),

            CalendarUnit::Half(year, half) => Interval::from_start(
                NaiveDate::from_ymd(*year, (*half * 6 - 5).try_into().unwrap(), 1),
                RelativeDuration::months(6),
            ),

            CalendarUnit::Month(year, month) => Interval::from_start(
                NaiveDate::from_ymd(*year, (*month).try_into().unwrap(), 1),
                RelativeDuration::months(1),
            ),

            CalendarUnit::Week(year, week) => Interval::from_start(
                NaiveDate::from_isoywd(*year, (*week).into(), chrono::Weekday::Mon),
                RelativeDuration::days(7),
            ),
        }
    }

    pub fn succ(&self) -> CalendarUnit {
        match self {
            CalendarUnit::Year(year) => CalendarUnit::Year(year + 1),
            CalendarUnit::Quarter(year, quarter) => {
                let mut quarter = *quarter;
                let mut year = *year;
                if quarter == 4 {
                    quarter = 1;
                    year = year + 1;
                }
                CalendarUnit::Quarter(year, quarter)
            }
            CalendarUnit::Half(_, _) => todo!(),
            CalendarUnit::Month(_, _) => todo!(),
            CalendarUnit::Week(_, _) => todo!(),
        }
    }
}

impl Display for CalendarUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CalendarUnit::Year(y) => write!(f, "{}", y),
            CalendarUnit::Quarter(y, q) => write!(f, "{}-Q{}", y, q),
            CalendarUnit::Half(y, h) => write!(f, "{}-H{}", y, h),
            CalendarUnit::Month(y, m) => write!(f, "{}-{}", y, m),
            CalendarUnit::Week(y, w) => write!(f, "{}-W{}", y, w),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::interval::marker::{End, Start};

    use super::*;

    #[test]
    fn test_quarter_interval() {
        let interval = CalendarUnit::Quarter(2022, 1).into_interval();
        assert_eq!(interval.start(), NaiveDate::from_ymd(2022, 1, 1));
        assert_eq!(interval.end(), NaiveDate::from_ymd(2022, 3, 31));

        let interval = CalendarUnit::Quarter(2022, 2).into_interval();
        assert_eq!(interval.start(), NaiveDate::from_ymd(2022, 4, 1));
        assert_eq!(interval.end(), NaiveDate::from_ymd(2022, 6, 30));
    }

    #[test]
    fn test_half_interval() {
        let interval = CalendarUnit::Half(2022, 2).into_interval();
        assert_eq!(interval.start(), NaiveDate::from_ymd(2022, 7, 1));
        assert_eq!(interval.end(), NaiveDate::from_ymd(2022, 12, 31));
    }
}
