use crate::{duration::RelativeDuration, IntervalLike};

use super::{bound::Bound, iter::UntilAfter};
use chrono::NaiveDate;
use std::fmt;

/// Iteration Error
#[derive(Debug)]
pub struct ImpossibleIterator;

impl fmt::Display for ImpossibleIterator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "open intervals cannot be iterated upon because it is infinite on one end"
        )
    }
}

/// An interval that is constructed off of the idea of the standard calendar (Gregorian Proleptic
/// calendar).
///
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
pub struct Interval {
    /// Indicating up to OR on in the direction of the interval
    ///
    /// e.g. if the direction is "forwards" and the end is inclusive then it will include the
    /// specified end date
    date: NaiveDate,
    duration: RelativeDuration,
}

impl Interval {
    /// Create an interval from a start and a duration
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::NaiveDate;
    /// use calends::{Interval, IntervalLike, RelativeDuration};
    ///
    /// let start = NaiveDate::from_ymd(2022, 1, 1);
    /// let duration = RelativeDuration::months(1);
    ///
    /// let mut interval = Interval::from_start(start, duration);
    ///
    /// assert_eq!(interval.start_date(), start);
    /// assert_eq!(interval.end_date(), NaiveDate::from_ymd(2022, 1, 31));
    /// ```
    pub fn from_start(date: NaiveDate, duration: RelativeDuration) -> Self {
        Interval { date, duration }
    }

    /// Create an interval from an end and a duration
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::NaiveDate;
    /// use calends::{Interval, IntervalLike, RelativeDuration};
    ///
    /// let interval = Interval::from_end(
    ///     NaiveDate::from_ymd(2022, 1, 1),
    ///     RelativeDuration::months(1).with_weeks(-2).with_days(2),
    /// );
    ///
    /// assert_eq!(interval.start_date(), NaiveDate::from_ymd(2021, 12, 13));
    /// assert_eq!(interval.end_date(), NaiveDate::from_ymd(2021, 12, 31));
    /// ```
    pub fn from_end(end: NaiveDate, duration: RelativeDuration) -> Self {
        Interval {
            date: end + -duration,
            duration,
        }
    }

    /// Create an interval with a specified set of dates
    pub fn with_dates(date: NaiveDate, _end: NaiveDate) -> Self {
        // TODO: determine the duration based on the delta
        Interval {
            date,
            duration: RelativeDuration::months(1),
        }
    }

    /// Start date of the interval
    pub fn start_date(&self) -> NaiveDate {
        self.date
    }

    /// End date of the interval
    pub fn end_date(&self) -> NaiveDate {
        (self.date + self.duration).pred()
    }

    /// Determine whether or not a date is inside of an interval
    pub fn within(&self, date: NaiveDate) -> bool {
        date >= self.start_date() && date <= self.end_date()
    }

    /// ISO8601-2:2019 Formatting of intervals
    ///
    /// The standard allows for:
    ///
    /// ```ignore
    ///
    /// - tiseE =[dtE]["/"][dtE]
    /// - tisdE = [dtE]["/"][duration]
    /// - tisdE = [duration]["/"][dtE]
    ///
    /// ```
    ///
    /// Currently we only represent the top one
    ///
    pub fn iso8601(&self) -> String {
        let start = self.date.to_string();
        let end = (self.date + self.duration).to_string();

        format!("{}/{}", start, end)
    }

    pub fn until_after(self, until: NaiveDate) -> UntilAfter<Interval> {
        UntilAfter::new(self, until)
    }
}

impl Iterator for Interval {
    type Item = Interval;

    fn next(&mut self) -> Option<Self::Item> {
        let interval = Interval::from_start(self.date, self.duration);
        self.date = self.date + self.duration;
        Some(interval)
    }
}

impl IntervalLike for Interval {
    fn start(&self) -> Bound<NaiveDate> {
        Bound::Included(self.start_date())
    }

    fn end(&self) -> Bound<NaiveDate> {
        Bound::Included(self.end_date())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_start() {
        let mut iter = Interval::from_start(
            NaiveDate::from_ymd(2022, 1, 1),
            RelativeDuration::months(1).with_weeks(-2).with_days(2),
        )
        .until_after(NaiveDate::from_ymd(2023, 1, 1));

        let next = iter.next().unwrap();
        assert_eq!(next.start_date(), NaiveDate::from_ymd(2022, 1, 1));
        assert_eq!(next.end_date(), NaiveDate::from_ymd(2022, 1, 19));

        let next = iter.next().unwrap();
        assert_eq!(next.start_date(), NaiveDate::from_ymd(2022, 1, 20));
        assert_eq!(next.end_date(), NaiveDate::from_ymd(2022, 2, 7));

        let next = iter.next().unwrap();
        assert_eq!(next.start_date(), NaiveDate::from_ymd(2022, 2, 8));
    }

    #[test]
    fn test_from_end() {
        let interval = Interval::from_end(
            NaiveDate::from_ymd(2022, 1, 1),
            RelativeDuration::months(1).with_weeks(-2).with_days(2),
        );

        assert_eq!(interval.start_date(), NaiveDate::from_ymd(2021, 12, 13));
        assert_eq!(interval.end_date(), NaiveDate::from_ymd(2021, 12, 31));
    }
}
