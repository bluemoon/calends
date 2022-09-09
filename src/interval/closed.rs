use crate::{duration::RelativeDuration, IntervalLike};

use super::{bound::Bound, iter::UntilAfter, marker, serde::SerializeInterval};
use chrono::NaiveDate;
use serde::{Serialize, Serializer};

/// An interval that is constructed off of the idea of the standard calendar (Gregorian Proleptic
/// calendar).
///
/// ## Interval creation rules
///
/// These rules have been adapted from ISO 8601-2:2019 7.14 Time Intervals.
///
/// - **Start and end:** A given start and end, we will calculate a duration based on the difference of
/// these two time sets and assign that as the duration. This may not produce the correct results
/// in the case of months.
///
/// - **Start and duration:** The start time plus the duration creates the end of the interval.
/// Intervals are then iterated on with the given duration. e.g. if the duration is 1 month, then
/// the next call to the iterator would give you a month in the future.
/// - **End and duration:** The end time minus the duration creates the beginning of the interval.
/// Intervals are then iterated on with the given duration.
///
/// ## Other notes
///
/// - This interval is by default inclusive on both ends.
///
/// # Rationale
///
/// We use this over [std::ops::Bound] because bound supports exclusive boundaries and we have made the
/// decision that it adds too much cognitive load / API cruft so we do not include it.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BoundInterval {
    /// Indicating up to OR on in the direction of the interval
    ///
    /// e.g. if the direction is "forwards" and the end is inclusive then it will include the
    /// specified end date
    date: NaiveDate,
    duration: RelativeDuration,
}

impl BoundInterval {
    /// Create an interval from a start and a duration
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::NaiveDate;
    /// use calends::{Interval, IntervalLike, RelativeDuration};
    /// use calends::interval::marker::{End, Start};
    ///
    /// let start = NaiveDate::from_ymd(2022, 1, 1);
    /// let duration = RelativeDuration::months(1);
    ///
    /// let mut interval = Interval::from_start(start, duration);
    ///
    /// assert_eq!(interval.start(), start);
    /// assert_eq!(interval.end(), NaiveDate::from_ymd(2022, 1, 31));
    /// ```
    pub fn from_start(date: NaiveDate, duration: RelativeDuration) -> Self {
        BoundInterval { date, duration }
    }

    /// Create an interval from an end and a duration
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::NaiveDate;
    /// use calends::{Interval, IntervalLike, RelativeDuration};
    /// use calends::interval::marker::{End, Start};
    ///
    /// let interval = Interval::from_end(
    ///     NaiveDate::from_ymd(2022, 1, 1),
    ///     RelativeDuration::months(1).with_weeks(-2).with_days(2),
    /// );
    ///
    /// assert_eq!(interval.start(), NaiveDate::from_ymd(2021, 12, 13));
    /// assert_eq!(interval.end(), NaiveDate::from_ymd(2021, 12, 31));
    /// ```
    pub fn from_end(end: NaiveDate, duration: RelativeDuration) -> Self {
        BoundInterval {
            date: end + -duration,
            duration,
        }
    }

    /// Create an interval with a specified set of dates
    pub fn with_dates(start: NaiveDate, end: NaiveDate) -> Self {
        BoundInterval {
            date: start,
            duration: RelativeDuration::from_duration_between(start, end),
        }
    }

    /// Start date of the interval
    fn computed_start_date(&self) -> NaiveDate {
        self.date
    }

    /// End date of the interval
    fn computed_end_date(&self) -> NaiveDate {
        (self.date + self.duration).pred()
    }

    pub fn until_after(self, until: NaiveDate) -> UntilAfter<BoundInterval> {
        UntilAfter::new(self, until)
    }
}

impl IntervalLike for BoundInterval {
    fn bound_start(&self) -> Bound<NaiveDate> {
        Bound::Included(self.computed_start_date())
    }

    fn bound_end(&self) -> Bound<NaiveDate> {
        Bound::Included(self.computed_end_date())
    }
}

impl marker::Start for BoundInterval {}
impl marker::End for BoundInterval {}

impl Serialize for BoundInterval {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        SerializeInterval(self.clone()).serialize(serializer)
    }
}

impl Iterator for BoundInterval {
    type Item = BoundInterval;

    fn next(&mut self) -> Option<Self::Item> {
        let interval = BoundInterval::from_start(self.date, self.duration);
        // to prevent overlapping dates we add one day
        self.date = self.date + self.duration;
        Some(interval)
    }
}

#[cfg(test)]
mod tests {
    use crate::interval::marker::{End, Start};

    use super::*;

    #[test]
    fn test_from_start() {
        let mut iter =
            BoundInterval::from_start(NaiveDate::from_ymd(2022, 1, 1), RelativeDuration::months(1))
                .until_after(NaiveDate::from_ymd(2023, 1, 1));

        let next = iter.next().unwrap();
        assert_eq!(next.start(), NaiveDate::from_ymd(2022, 1, 1));
        assert_eq!(next.end(), NaiveDate::from_ymd(2022, 1, 31));

        let next = iter.next().unwrap();
        assert_eq!(next.start(), NaiveDate::from_ymd(2022, 2, 1));
        assert_eq!(next.end(), NaiveDate::from_ymd(2022, 2, 28));

        let next = iter.next().unwrap();
        assert_eq!(next.start(), NaiveDate::from_ymd(2022, 3, 1));
    }

    #[test]
    fn test_from_end() {
        let interval = BoundInterval::from_end(
            NaiveDate::from_ymd(2022, 1, 1),
            RelativeDuration::months(1).with_weeks(-2).with_days(2),
        );

        assert_eq!(interval.start(), NaiveDate::from_ymd(2021, 12, 13));
        assert_eq!(interval.end(), NaiveDate::from_ymd(2021, 12, 31));
    }
}
