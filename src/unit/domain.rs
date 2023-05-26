use std::fmt::Display;

use chrono::NaiveDate;
use serde::{Serialize, Serializer};

use crate::{interval::ClosedInterval, Interval, RelativeDuration};

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
/// This will also likely be useful if custom fiscal calendars ever get added
///
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum CalendarUnit {
    Year(i32),
    Quarter(i32, u8),
    Half(i32, u8),
    Month(i32, u8),
    Week(i32, u8),
}

impl CalendarUnit {
    pub fn into_interval(&self) -> Interval {
        let res = match self {
            CalendarUnit::Year(year) => ClosedInterval::from_start(
                NaiveDate::from_yo_opt(*year, 1).unwrap(),
                RelativeDuration::months(12).with_days(-1),
            ),
            CalendarUnit::Quarter(year, quarter) => ClosedInterval::from_start(
                NaiveDate::from_ymd_opt(*year, (*quarter * 3 - 2).try_into().unwrap(), 1).unwrap(),
                RelativeDuration::months(3).with_days(-1),
            ),

            CalendarUnit::Half(year, half) => ClosedInterval::from_start(
                NaiveDate::from_ymd_opt(*year, (*half * 6 - 5).try_into().unwrap(), 1).unwrap(),
                RelativeDuration::months(6).with_days(-1),
            ),

            CalendarUnit::Month(year, month) => ClosedInterval::from_start(
                NaiveDate::from_ymd_opt(*year, (*month).try_into().unwrap(), 1).unwrap(),
                RelativeDuration::months(1).with_days(-1),
            ),

            CalendarUnit::Week(year, week) => ClosedInterval::from_start(
                NaiveDate::from_isoywd_opt(*year, (*week).into(), chrono::Weekday::Mon).unwrap(),
                RelativeDuration::days(7),
            ),
        };

        Interval::Closed(res)
    }

    pub fn succ(&self) -> CalendarUnit {
        match self {
            CalendarUnit::Year(year) => CalendarUnit::Year(year + 1),
            CalendarUnit::Quarter(year, quarter) => {
                let mut quarter = *quarter;
                let mut year = *year;
                if quarter == 4 {
                    quarter = 1;
                    year += 1;
                } else {
                    quarter += 1;
                }
                CalendarUnit::Quarter(year, quarter)
            }
            CalendarUnit::Half(year, half) => {
                let mut half = *half;
                let mut year = *year;
                if half == 2 {
                    half = 1;
                    year += 1;
                } else {
                    half += 1;
                }
                CalendarUnit::Half(year, half)
            }
            CalendarUnit::Month(year, month) => {
                let mut month = *month;
                let mut year = *year;
                if month == 12 {
                    month = 1;
                    year += 1;
                } else {
                    month += 1;
                }
                CalendarUnit::Month(year, month)
            }
            CalendarUnit::Week(_, _) => todo!(),
        }
    }
}

impl Iterator for CalendarUnit {
    type Item = CalendarUnit;

    fn next(&mut self) -> Option<Self::Item> {
        let cur = *self;
        *self = cur.succ();
        Some(cur)
    }
}

impl Display for CalendarUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CalendarUnit::Year(y) => write!(f, "{}", y),
            CalendarUnit::Quarter(y, q) => write!(f, "{}-Q{}", y, q),
            CalendarUnit::Half(y, h) => write!(f, "{}-H{}", y, h),
            CalendarUnit::Month(y, m) => write!(f, "{}-{:0>2}", y, m),
            CalendarUnit::Week(y, w) => write!(f, "{}-W{:0>2}", y, w),
        }
    }
}

/// Serialize a `CalendarUnit`
impl Serialize for CalendarUnit {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::IntervalLike;

    use super::*;

    #[test]
    fn test_quarter_iterator() {
        let mut c = CalendarUnit::Quarter(2022, 1);
        assert_eq!(c.next(), Some(CalendarUnit::Quarter(2022, 1)));
        assert_eq!(c.next(), Some(CalendarUnit::Quarter(2022, 2)));
        assert_eq!(c.next(), Some(CalendarUnit::Quarter(2022, 3)));
        assert_eq!(c.next(), Some(CalendarUnit::Quarter(2022, 4)));
        assert_eq!(c.next(), Some(CalendarUnit::Quarter(2023, 1)));
    }

    #[test]
    fn test_half_iterator() {
        let mut c = CalendarUnit::Half(2022, 1);
        assert_eq!(c.next(), Some(CalendarUnit::Half(2022, 1)));
        assert_eq!(c.next(), Some(CalendarUnit::Half(2022, 2)));
        assert_eq!(c.next(), Some(CalendarUnit::Half(2023, 1)));
    }

    #[test]
    fn test_quarter_interval() {
        let interval = CalendarUnit::Quarter(2022, 1).into_interval();
        assert_eq!(
            interval.start_opt().unwrap(),
            NaiveDate::from_ymd_opt(2022, 1, 1).unwrap()
        );
        assert_eq!(
            interval.end_opt().unwrap(),
            NaiveDate::from_ymd_opt(2022, 3, 31).unwrap()
        );

        let interval = CalendarUnit::Quarter(2022, 2).into_interval();
        assert_eq!(
            interval.start_opt().unwrap(),
            NaiveDate::from_ymd_opt(2022, 4, 1).unwrap()
        );
        assert_eq!(
            interval.end_opt().unwrap(),
            NaiveDate::from_ymd_opt(2022, 6, 30).unwrap()
        );
    }

    #[test]
    fn test_half_interval() {
        let interval = CalendarUnit::Half(2022, 2).into_interval();
        assert_eq!(
            interval.start_opt().unwrap(),
            NaiveDate::from_ymd_opt(2022, 7, 1).unwrap()
        );
        assert_eq!(
            interval.end_opt().unwrap(),
            NaiveDate::from_ymd_opt(2022, 12, 31).unwrap()
        );
    }
}
