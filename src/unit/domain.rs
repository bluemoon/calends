use std::fmt::Display;

use chrono::NaiveDate;

use crate::{Interval, RelativeDuration};

/// A unit in time
#[derive(Debug, PartialEq, Eq)]
pub enum CalendarUnit {
    /// Quarter can be represented as:
    ///
    /// ```
    /// CalendarUnit::Quarter(2022, 1)
    /// ```
    Quarter(i32, u8),
    Half(i32, u8),
    Month(i32, u8),
    Week(i32, u8),
}

impl CalendarUnit {
    pub fn into_interval(&self) -> Interval {
        match self {
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
}

impl Display for CalendarUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CalendarUnit::Quarter(y, q) => write!(f, "{}-Q{}", y, q),
            CalendarUnit::Half(_, _) => todo!(),
            CalendarUnit::Month(_, _) => todo!(),
            CalendarUnit::Week(_, _) => todo!(),
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
