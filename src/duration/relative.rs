//! Implement a Duration that extends chrono and adds Quarter and Month
use std::ops::{Add, Div, Mul, Neg, Sub};

use chrono::{Datelike, NaiveDate};
use modular_bitfield::bitfield;
use modular_bitfield::prelude::B20;

use crate::shift;

#[bitfield]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RelativeImpl {
    pub months: B20,
    pub weeks: B20,
    pub days: B20,
    pub months_negative: bool,
    pub weeks_negative: bool,
    pub days_negative: bool,
    pub pad: bool,
}

/// A duration of time which can be positive or negative
///
/// # Rationale
///
/// Using ISO8601-2:2019 and CalConnect CC 18011 as guidelines, we have formulated a bitpacked
/// duration. This duration supports months, weeks and days
///
/// # Limitations
///
/// Chrono DateImpl only supports 13 bits for years so around 8000 years
///
///
/// ```text
///
/// ┌─────┐                                                      
/// │ MSB │                                        ┌────────────┐   
/// └┬────┘                                        │ Neg. Flag  │◀┐
///  │                                             └────────────┘ │
///  ▼                                                            │
/// ┌──────────────────┬──────────────────┬──────────────────┬────┴┐
/// │Years (20 bits)   │Weeks (20 bits)   │Days (20 bits)    │     │
/// └──────────────────┴──────────────────┴──────────────────┴─────┘
///       ◀ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─
///
/// ```
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct RelativeDuration(RelativeImpl);

impl RelativeDuration {
    /// Returns a RelativeDuration for a given set of dates
    ///
    /// Calculate the difference between two sets of dates and return back a duration
    ///
    /// # Examples
    ///
    /// ```
    /// # use calends::RelativeDuration;
    /// # use chrono::NaiveDate;
    ///
    /// let duration = RelativeDuration::from_duration_between(
    ///     NaiveDate::from_ymd(2022, 1, 1),
    ///     NaiveDate::from_ymd(2023, 1, 1),
    ///  );
    ///
    /// assert_eq!(
    ///     NaiveDate::from_ymd(2022, 1, 1) + duration,
    ///     NaiveDate::from_ymd(2023, 1, 1)
    /// );
    /// ```
    pub fn from_duration_between(start: NaiveDate, end: NaiveDate) -> RelativeDuration {
        let mut months = (end.year() - start.year()) * 12;
        months += (end.month() - start.month()) as i32;
        let days = (end.day() - start.day()) as i32;
        RelativeDuration::from_raw(months, 0, days).unwrap()
    }

    pub fn from_mwd(months: i32, weeks: i32, days: i32) -> RelativeDuration {
        RelativeDuration::from_raw(months, weeks, days)
            .expect("relative duration is invalid and exceeds bounds")
    }

    fn from_raw(months: i32, weeks: i32, days: i32) -> Option<RelativeDuration> {
        Some(
            RelativeDuration(RelativeImpl::default())
                .with_months(months)
                .with_weeks(weeks)
                .with_days(days),
        )
    }

    /// Create a RelativeDuration with the number of months
    pub fn months(months: i32) -> RelativeDuration {
        RelativeDuration::default().with_months(months)
    }

    /// Create a RelativeDuration with the numer of weeks
    pub fn weeks(weeks: i32) -> RelativeDuration {
        RelativeDuration::default().with_weeks(weeks)
    }

    /// Create a RelativeDuration with the number of days
    #[inline]
    pub fn days(days: i32) -> RelativeDuration {
        RelativeDuration::default().with_days(days)
    }

    /// Set the number of months in the duration
    #[inline]
    pub fn with_months(&self, months: i32) -> RelativeDuration {
        let RelativeDuration(mut ri) = self;
        if months.is_negative() {
            ri = ri.with_months(-months as u32);
            ri = ri.with_months_negative(true);
        } else {
            ri = ri.with_months(months as u32);
            ri = ri.with_months_negative(false);
        }
        RelativeDuration(ri)
    }

    /// Number of months in the duration
    #[inline]
    pub fn num_months(&self) -> i32 {
        let months = self.0.months() as i32;
        if self.0.months_negative() {
            -months
        } else {
            months
        }
    }

    /// Number of weeks in the duration
    #[inline]
    pub fn num_weeks(&self) -> i32 {
        let weeks = self.0.weeks() as i32;
        if self.0.weeks_negative() {
            -weeks
        } else {
            weeks
        }
    }

    /// Set the number of months in the duration
    #[inline]
    pub fn with_weeks(&self, weeks: i32) -> RelativeDuration {
        let RelativeDuration(mut ri) = self;
        if weeks.is_negative() {
            ri = ri.with_weeks(-weeks as u32);
            ri = ri.with_weeks_negative(true);
        } else {
            ri = ri.with_weeks(weeks as u32);
            ri = ri.with_weeks_negative(false);
        }
        RelativeDuration(ri)
    }

    /// Number of weeks in the duration
    #[inline]
    pub fn num_days(&self) -> i32 {
        let days = self.0.days() as i32;
        if self.0.days_negative() {
            -days
        } else {
            days
        }
    }

    /// Set the number of days in the duration
    #[inline]
    pub fn with_days(&self, days: i32) -> RelativeDuration {
        let RelativeDuration(mut ri) = self;
        if days.is_negative() {
            ri = ri.with_days(-days as u32);
            ri = ri.with_days_negative(true);
        } else {
            ri = ri.with_days(days as u32);
            ri = ri.with_days_negative(false);
        }
        RelativeDuration(ri)
    }

    /// A `RelativeDuration` representing zero.
    #[inline]
    pub fn zero() -> RelativeDuration {
        RelativeDuration::from_mwd(0, 0, 0)
    }

    /// Returns true if the duration equals RelativeDuration::zero().
    #[inline]
    pub fn is_zero(&self) -> bool {
        self.num_months() == 0 && self.num_weeks() == 0 && self.num_days() == 0
    }

    /// Return an ISO8601-2:2019 formatted duration, notably we do not include offsets for time
    /// (hours, minutes or seconds etc.)
    ///
    /// # Examples of output
    ///
    /// - 'P5D' is a duration of 5 days
    /// - 'P120M400D' is a duration of 120 months and 400 days
    /// - 'P4W3D' is a duration of 4 weeks and 3 days
    /// - 'P-4M3W' is a duration of negative 4 months and positive 3 weeks, the minus sign can be
    /// applied to each of the components within the serialization format
    ///
    pub fn iso8601(&self) -> String {
        let build = vec![
            (self.num_months(), "M"),
            (self.num_weeks(), "W"),
            (self.num_days(), "D"),
        ];

        let mut result = String::from("P");

        for (count, unit) in build.iter() {
            if *count != 0 {
                result.push_str(&count.to_string());
                result.push_str(unit);
            }
        }

        result
    }
}

impl PartialOrd for RelativeDuration {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        (self.num_months(), self.num_weeks(), self.num_days()).partial_cmp(&(
            other.num_months(),
            other.num_weeks(),
            other.num_days(),
        ))
    }
}

impl Ord for RelativeDuration {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.num_months(), self.num_weeks(), self.num_days()).cmp(&(
            other.num_months(),
            other.num_weeks(),
            other.num_days(),
        ))
    }
}

impl Neg for RelativeDuration {
    type Output = RelativeDuration;

    #[inline]
    fn neg(self) -> RelativeDuration {
        let RelativeDuration(mut ri) = self;
        ri = ri.with_months_negative(!ri.months_negative());
        ri = ri.with_weeks_negative(!ri.weeks_negative());
        ri = ri.with_days_negative(!ri.days_negative());
        RelativeDuration(ri)
    }
}

impl Add<RelativeDuration> for RelativeDuration {
    type Output = RelativeDuration;

    #[inline]
    fn add(self, rhs: RelativeDuration) -> RelativeDuration {
        RelativeDuration::from_mwd(
            self.num_months() + rhs.num_months(),
            self.num_weeks() + rhs.num_weeks(),
            self.num_days() + rhs.num_days(),
        )
    }
}

impl Sub for RelativeDuration {
    type Output = RelativeDuration;

    #[inline]
    fn sub(self, rhs: RelativeDuration) -> RelativeDuration {
        self + (-rhs)
    }
}

impl Mul<i32> for RelativeDuration {
    type Output = RelativeDuration;

    #[inline]
    fn mul(self, rhs: i32) -> RelativeDuration {
        RelativeDuration::from_mwd(
            self.num_months() * rhs,
            self.num_weeks() * rhs,
            self.num_days() * rhs,
        )
    }
}

impl Div<i32> for RelativeDuration {
    type Output = RelativeDuration;

    #[inline]
    fn div(self, rhs: i32) -> RelativeDuration {
        RelativeDuration::from_mwd(
            self.num_months() / rhs,
            self.num_weeks() / rhs,
            self.num_days() / rhs,
        )
    }
}

/// Add a duration to a [NaiveDate]
///
/// Precendence for adding is from largest unit to smallest unit
impl Add<RelativeDuration> for NaiveDate {
    type Output = NaiveDate;

    #[inline]
    fn add(self, rhs: RelativeDuration) -> NaiveDate {
        let date = shift::shift_months(self, rhs.num_months());
        let date = shift::shift_weeks(date, rhs.num_weeks());
        shift::shift_days(date, rhs.num_days())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_duration_transits_year() {
        let duration = RelativeDuration::from_duration_between(
            NaiveDate::from_ymd(2022, 1, 1),
            NaiveDate::from_ymd(2023, 1, 1),
        );

        assert_eq!(
            NaiveDate::from_ymd(2022, 1, 1) + duration,
            NaiveDate::from_ymd(2023, 1, 1)
        );
    }

    #[test]
    fn test_from_duration_transits_month() {
        let duration = RelativeDuration::from_duration_between(
            NaiveDate::from_ymd(2023, 3, 1),
            NaiveDate::from_ymd(2023, 3, 31),
        );

        assert_eq!(
            NaiveDate::from_ymd(2023, 3, 1) + duration,
            NaiveDate::from_ymd(2023, 3, 31)
        );
    }

    #[test]
    fn test_from_duration_transits_months_and_days() {
        let duration = RelativeDuration::from_duration_between(
            NaiveDate::from_ymd(2023, 3, 1),
            NaiveDate::from_ymd(2023, 4, 20),
        );

        assert_eq!(
            NaiveDate::from_ymd(2023, 3, 1) + duration,
            NaiveDate::from_ymd(2023, 4, 20)
        );
    }

    #[test]
    fn test_from_duration_between_year() {
        let duration = RelativeDuration::from_duration_between(
            NaiveDate::from_ymd(2022, 1, 1),
            NaiveDate::from_ymd(2023, 1, 1),
        );

        assert_eq!(duration.num_months(), 12);
        assert_eq!(duration.num_weeks(), 0);
        assert_eq!(duration.num_days(), 0);
    }

    #[test]
    fn test_from_duration_between_month() {
        let duration = RelativeDuration::from_duration_between(
            NaiveDate::from_ymd(2022, 1, 1),
            NaiveDate::from_ymd(2022, 2, 1),
        );

        assert_eq!(duration.num_months(), 1);
        assert_eq!(duration.num_days(), 0);
    }

    #[test]
    fn test_display() {
        assert_eq!(
            RelativeDuration::weeks(4)
                .with_months(4)
                .with_days(32)
                .to_string(),
            String::from("4 months 4 weeks 32 days")
        );

        // XXX: does this even make sense?
        assert_eq!(RelativeDuration::zero().to_string(), String::from(""));

        assert_eq!(
            RelativeDuration::weeks(1)
                .with_months(1)
                .with_days(1)
                .to_string(),
            String::from("1 month 1 week 1 day")
        );

        assert_eq!(
            RelativeDuration::weeks(-1)
                .with_months(1)
                .with_days(1)
                .to_string(),
            String::from("1 month -1 week 1 day")
        )
    }

    #[test]
    fn test_iso8601() {
        // - 'P5D' is a duration of 5 days
        assert_eq!(RelativeDuration::days(5).iso8601(), "P5D");
        // - 'P120M400D' is a duration of 120 months and 400 days
        assert_eq!(
            RelativeDuration::months(120).with_days(400).iso8601(),
            "P120M400D"
        );
        // - 'P-4M3W' is a duration of negative 4 months and positive 3 weeks, the minus sign can be
        // applied to each of the components within the serialization format
        assert_eq!(
            RelativeDuration::months(-4).with_weeks(3).iso8601(),
            "P-4M3W"
        );
    }

    #[test]
    fn test_zero() {
        assert!(RelativeDuration::zero().is_zero());
    }

    #[test]
    fn test_negate() {
        assert_eq!((-RelativeDuration::months(1)).num_months(), -1);
    }

    #[test]
    fn test_subtract() {
        assert_eq!(
            RelativeDuration::months(1).with_weeks(1).with_days(1)
                - RelativeDuration::months(1).with_weeks(1).with_days(1),
            RelativeDuration::zero()
        );
    }

    #[test]
    fn test_mul() {
        assert_eq!(
            RelativeDuration::months(1).with_weeks(1).with_days(1) * 2,
            RelativeDuration::months(2).with_weeks(2).with_days(2)
        );
    }

    #[test]
    fn test_div() {
        assert_eq!(
            RelativeDuration::months(2).with_weeks(2).with_days(2) / 2,
            RelativeDuration::months(1).with_weeks(1).with_days(1)
        );
    }

    #[test]
    fn test_month() {
        assert_eq!(RelativeDuration::months(1).num_months(), 1);
        assert_eq!(RelativeDuration::months(-1).num_months(), -1)
    }

    #[test]
    fn test_week() {
        assert_eq!(RelativeDuration::weeks(1).num_weeks(), 1);
        assert_eq!(RelativeDuration::weeks(-1).num_weeks(), -1)
    }

    #[test]
    fn test_day() {
        assert_eq!(RelativeDuration::days(1).num_days(), 1);
        assert_eq!(RelativeDuration::days(-1).num_days(), -1)
    }

    #[test]
    fn test_add_year() {
        let rd = RelativeDuration::months(12);
        let next = NaiveDate::from_ymd(2022, 1, 1) + rd;
        assert_eq!(next, NaiveDate::from_ymd(2023, 1, 1));
    }
}
