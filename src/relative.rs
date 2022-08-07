//! Implement a Duration that extends chrono and adds Quarter and Month
use std::ops::{Add, Bound, Div, Mul, Neg, Sub};

use chrono::{Duration, NaiveDate};

use crate::{interval::naive::NaiveInterval, shift};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RelativeDuration {
    months: i32,
    duration: Duration,
}

impl From<Duration> for RelativeDuration {
    /// Makes a new `RelativeDuration` from a `chrono::Duration`.
    #[inline]
    fn from(item: Duration) -> Self {
        RelativeDuration {
            months: 0,
            duration: item,
        }
    }
}

impl RelativeDuration {
    /// Makes a new `RelativeDuration` with given number of years.
    ///
    /// Equivalent to `RelativeDuration::months(years * 12)` with overflow checks.
    /// Panics when the duration is out of bounds.
    #[inline]
    pub fn years(years: i32) -> RelativeDuration {
        let months = years
            .checked_mul(12)
            .expect("RelativeDuration::years out of bounds");
        RelativeDuration::months(months)
    }

    /// Makes a new `RelativeDuration` with given number of months.
    /// Panics when the duration is out of bounds.
    #[inline]
    pub fn months(months: i32) -> RelativeDuration {
        RelativeDuration {
            months,
            duration: Duration::zero(),
        }
    }

    /// Makes a new `RelativeDuration` with given number of weeks.
    /// Panics when the duration is out of bounds.
    #[inline]
    pub fn weeks(weeks: i64) -> RelativeDuration {
        RelativeDuration {
            months: 0,
            duration: Duration::weeks(weeks),
        }
    }

    /// Makes a new `RelativeDuration` with given number of days.
    /// Panics when the duration is out of bounds.
    #[inline]
    pub fn days(days: i64) -> RelativeDuration {
        RelativeDuration {
            months: 0,
            duration: Duration::days(days),
        }
    }

    /// Update the `Duration` part of the current `RelativeDuration`.
    #[inline]
    pub fn with_duration(self, duration: Duration) -> RelativeDuration {
        RelativeDuration {
            months: self.months,
            duration,
        }
    }

    /// A `RelativeDuration` representing zero.
    #[inline]
    pub fn zero() -> RelativeDuration {
        RelativeDuration {
            months: 0,
            duration: Duration::zero(),
        }
    }

    /// Returns true if the duration equals RelativeDuration::zero().
    #[inline]
    pub fn is_zero(&self) -> bool {
        self.months == 0 && self.duration.is_zero()
    }

    pub fn into_interval(&self, start_date: NaiveDate) -> NaiveInterval {
        NaiveInterval::new(
            Bound::Included(start_date),
            Bound::Included(start_date + self.clone()),
        )
    }
}

impl Neg for RelativeDuration {
    type Output = RelativeDuration;

    #[inline]
    fn neg(self) -> RelativeDuration {
        RelativeDuration {
            months: -self.months,
            duration: -self.duration,
        }
    }
}

impl Add<RelativeDuration> for RelativeDuration {
    type Output = RelativeDuration;

    #[inline]
    fn add(self, rhs: RelativeDuration) -> RelativeDuration {
        RelativeDuration {
            months: self.months + rhs.months,
            duration: self.duration + rhs.duration,
        }
    }
}

impl Add<Duration> for RelativeDuration {
    type Output = RelativeDuration;

    #[inline]
    fn add(self, rhs: Duration) -> RelativeDuration {
        self + RelativeDuration {
            months: 0,
            duration: rhs,
        }
    }
}

impl Add<RelativeDuration> for Duration {
    type Output = RelativeDuration;

    #[inline]
    fn add(self, rhs: RelativeDuration) -> RelativeDuration {
        rhs + self
    }
}

impl Sub for RelativeDuration {
    type Output = RelativeDuration;

    #[inline]
    fn sub(self, rhs: RelativeDuration) -> RelativeDuration {
        self + (-rhs)
    }
}

impl Sub<RelativeDuration> for Duration {
    type Output = RelativeDuration;

    #[inline]
    fn sub(self, rhs: RelativeDuration) -> RelativeDuration {
        -rhs + self
    }
}

impl Sub<Duration> for RelativeDuration {
    type Output = RelativeDuration;

    #[inline]
    fn sub(self, rhs: Duration) -> RelativeDuration {
        self + (-rhs)
    }
}

impl Mul<i32> for RelativeDuration {
    type Output = RelativeDuration;

    #[inline]
    fn mul(self, rhs: i32) -> RelativeDuration {
        RelativeDuration {
            months: self.months * rhs,
            duration: self.duration * rhs,
        }
    }
}

impl Div<i32> for RelativeDuration {
    type Output = RelativeDuration;

    #[inline]
    fn div(self, rhs: i32) -> RelativeDuration {
        RelativeDuration {
            months: self.months / rhs,
            duration: self.duration / rhs,
        }
    }
}

// The following is just copy-pasta, mostly because we
// can't impl<T> Add<RelativeDuration> for T with T: Datelike
impl Add<RelativeDuration> for NaiveDate {
    type Output = NaiveDate;

    #[inline]
    fn add(self, rhs: RelativeDuration) -> NaiveDate {
        shift::shift_months(self, rhs.months) + rhs.duration
    }
}
