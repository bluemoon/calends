use chrono::{Datelike, NaiveDate};

use crate::{Interval, RelativeDuration};

/// Groupings of time
///
/// # Rationale
///
/// We as humans have come up with various groupings of time, the ones we have chosen to represent
/// here are:
///
/// - Half
/// - Quarter
/// - Month
/// - Week
///
/// ## How is this different than interval?
///
/// An interval can be be thought of as a duration of time that does not have a frame of reference.
/// It can loosely fit anywhere you want it, if you want a duration of 3 months and 7 days that is
/// perfectly fine.
///
/// A grouping tends to be a contiguous set of dates
///
#[derive(Debug, PartialEq, Eq)]
pub enum Grouping {
    Quarter(i32, i8),
    Half(i32, i8),
}

impl Grouping {
    pub fn from_date_for_quarter(date: NaiveDate) -> Self {
        Grouping::Quarter(
            date.year(),
            ((date.month() - 1) / 3 + 1).try_into().unwrap(),
        )
    }

    pub fn from_date_for_half(date: NaiveDate) -> Self {
        Grouping::Half(date.year(), (date.month() / 2 + 1).try_into().unwrap())
    }

    pub fn into_interval(&self) -> Interval {
        match self {
            Grouping::Quarter(year, quarter) => Interval::from_start(
                NaiveDate::from_ymd(*year, (*quarter * 3 - 2).try_into().unwrap(), 1),
                RelativeDuration::months(3),
            ),
            Grouping::Half(_, _) => todo!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::interval::marker::{End, Start};

    use super::*;

    #[test]
    fn test_group_quarter() {
        assert_eq!(
            Grouping::from_date_for_quarter(NaiveDate::from_ymd(2020, 2, 29)),
            Grouping::Quarter(2020, 1)
        );

        assert_eq!(
            Grouping::from_date_for_quarter(NaiveDate::from_ymd(2022, 12, 31)),
            Grouping::Quarter(2022, 4)
        )
    }

    #[test]
    fn test_group_quarter_interval() {
        let group = Grouping::from_date_for_quarter(NaiveDate::from_ymd(2020, 2, 29));
        let interval = group.into_interval();
        assert_eq!(interval.start(), NaiveDate::from_ymd(2020, 1, 1));
        assert_eq!(interval.end(), NaiveDate::from_ymd(2020, 3, 31));
    }
}
