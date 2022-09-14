use chrono::NaiveDate;

use crate::{IntervalLike, RelativeDuration};

use super::bound::Bound;
use super::closed::ClosedInterval;
use super::iter::UntilAfter;
use super::marker;
use super::open::{OpenEndInterval, OpenStartInterval};

#[derive(Debug, thiserror::Error)]
pub enum IntervalError {
    #[error("the variant is not iterable")]
    NotIterable,

    #[error("is not convertible to with start")]
    NotConvertibleToWithStart,

    #[error("is not convertible to with end")]
    NotConvertibleToWithEnd,
}

/// Inerval with three variants, closed, open start, open end
///
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
///
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum Interval {
    /// A closed interval that will always have a start and end
    Closed(ClosedInterval),
    OpenStart(OpenStartInterval),
    OpenEnd(OpenEndInterval),
}

impl Interval {
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
    /// let duration = RelativeDuration::months(1).with_days(-1);
    ///
    /// let mut interval = Interval::closed_from_start(start, duration);
    ///
    /// assert_eq!(interval.start_opt().unwrap(), start);
    /// assert_eq!(interval.end_opt().unwrap(), NaiveDate::from_ymd(2022, 1, 31));
    /// ```
    pub fn closed_from_start(date: NaiveDate, duration: RelativeDuration) -> Self {
        Interval::Closed(ClosedInterval::from_start(date, duration))
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
    /// let interval = Interval::closed_from_end(
    ///     NaiveDate::from_ymd(2022, 1, 1),
    ///     RelativeDuration::months(1).with_weeks(-2).with_days(2),
    /// );
    ///
    /// assert_eq!(interval.start_opt().unwrap(), NaiveDate::from_ymd(2021, 12, 13));
    /// assert_eq!(interval.end_opt().unwrap(), NaiveDate::from_ymd(2022, 1, 1));
    /// ```
    pub fn closed_from_end(end: NaiveDate, duration: RelativeDuration) -> Self {
        Interval::Closed(ClosedInterval::from_end(end, duration))
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
    /// let interval = Interval::closed_with_dates(
    ///     NaiveDate::from_ymd(2022, 1, 1),
    ///     NaiveDate::from_ymd(2023, 1, 1),
    /// );
    ///
    /// assert_eq!(interval.start_opt().unwrap(), NaiveDate::from_ymd(2022, 1, 1));
    /// assert_eq!(interval.end_opt().unwrap(), NaiveDate::from_ymd(2023, 1, 1));
    /// ```
    pub fn closed_with_dates(start: NaiveDate, end: NaiveDate) -> Self {
        Interval::Closed(ClosedInterval::with_dates(start, end))
    }

    pub fn open_start(end: NaiveDate) -> Self {
        Interval::OpenStart(OpenStartInterval::new(end))
    }

    pub fn open_end(start: NaiveDate) -> Self {
        Interval::OpenEnd(OpenEndInterval::new(start))
    }

    pub fn until_after(
        self,
        until: NaiveDate,
    ) -> Result<UntilAfter<ClosedInterval>, IntervalError> {
        match self {
            Interval::Closed(closed) => Ok(UntilAfter::new(closed, until)),
            Interval::OpenStart(_) => Err(IntervalError::NotIterable),
            Interval::OpenEnd(_) => Err(IntervalError::NotIterable),
        }
    }
}

impl IntervalLike for Interval {
    fn bound_start(&self) -> Bound<chrono::NaiveDate> {
        match self {
            Interval::Closed(c) => c.bound_start(),
            Interval::OpenStart(us) => us.bound_start(),
            Interval::OpenEnd(ue) => ue.bound_start(),
        }
    }

    fn bound_end(&self) -> Bound<chrono::NaiveDate> {
        match self {
            Interval::Closed(c) => c.bound_end(),
            Interval::OpenStart(us) => us.bound_end(),
            Interval::OpenEnd(ue) => ue.bound_end(),
        }
    }

    fn duration(&self) -> Option<RelativeDuration> {
        match self {
            Interval::Closed(i) => i.duration(),
            Interval::OpenStart(i) => i.duration(),
            Interval::OpenEnd(i) => i.duration(),
        }
    }
}

impl From<IntervalWithStart> for Interval {
    fn from(i: IntervalWithStart) -> Self {
        match i {
            IntervalWithStart::Closed(i) => Interval::Closed(i),
            IntervalWithStart::OpenEnd(i) => Interval::OpenEnd(i),
        }
    }
}

impl From<IntervalWithEnd> for Interval {
    fn from(i: IntervalWithEnd) -> Self {
        match i {
            IntervalWithEnd::Closed(i) => Interval::Closed(i),
            IntervalWithEnd::OpenStart(i) => Interval::OpenStart(i),
        }
    }
}

/// An interval that has a guaranteed start but deos not guarantee and end
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IntervalWithStart {
    Closed(ClosedInterval),
    OpenEnd(OpenEndInterval),
}

impl IntervalWithStart {
    pub fn until_after(
        self,
        until: NaiveDate,
    ) -> Result<UntilAfter<ClosedInterval>, IntervalError> {
        match self {
            IntervalWithStart::Closed(closed) => Ok(UntilAfter::new(closed, until)),
            IntervalWithStart::OpenEnd(_) => Err(IntervalError::NotIterable),
        }
    }
}

impl IntervalLike for IntervalWithStart {
    fn bound_start(&self) -> Bound<chrono::NaiveDate> {
        match self {
            IntervalWithStart::Closed(c) => c.bound_start(),
            IntervalWithStart::OpenEnd(ue) => ue.bound_start(),
        }
    }

    fn bound_end(&self) -> Bound<chrono::NaiveDate> {
        match self {
            IntervalWithStart::Closed(c) => c.bound_end(),
            IntervalWithStart::OpenEnd(ue) => ue.bound_end(),
        }
    }

    fn duration(&self) -> Option<RelativeDuration> {
        match self {
            IntervalWithStart::Closed(i) => i.duration(),
            IntervalWithStart::OpenEnd(i) => i.duration(),
        }
    }
}

impl marker::Start for IntervalWithStart {}

impl TryFrom<Interval> for IntervalWithStart {
    type Error = IntervalError;

    fn try_from(value: Interval) -> Result<Self, Self::Error> {
        match value {
            Interval::Closed(i) => Ok(IntervalWithStart::Closed(i)),
            Interval::OpenStart(_) => Err(IntervalError::NotConvertibleToWithStart),
            Interval::OpenEnd(i) => Ok(IntervalWithStart::OpenEnd(i)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IntervalWithEnd {
    Closed(ClosedInterval),
    OpenStart(OpenStartInterval),
}

impl IntervalLike for IntervalWithEnd {
    fn bound_start(&self) -> Bound<chrono::NaiveDate> {
        match self {
            IntervalWithEnd::Closed(c) => c.bound_start(),
            IntervalWithEnd::OpenStart(u) => u.bound_start(),
        }
    }

    fn bound_end(&self) -> Bound<chrono::NaiveDate> {
        match self {
            IntervalWithEnd::Closed(c) => c.bound_end(),
            IntervalWithEnd::OpenStart(u) => u.bound_end(),
        }
    }

    fn duration(&self) -> Option<RelativeDuration> {
        match self {
            IntervalWithEnd::Closed(i) => i.duration(),
            IntervalWithEnd::OpenStart(i) => i.duration(),
        }
    }
}

impl marker::End for IntervalWithEnd {}

impl TryFrom<Interval> for IntervalWithEnd {
    type Error = IntervalError;

    fn try_from(value: Interval) -> Result<Self, Self::Error> {
        match value {
            Interval::Closed(i) => Ok(IntervalWithEnd::Closed(i)),
            Interval::OpenEnd(_) => Err(IntervalError::NotConvertibleToWithEnd),
            Interval::OpenStart(i) => Ok(IntervalWithEnd::OpenStart(i)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reciprocity() {
        let start = NaiveDate::from_ymd(2022, 1, 1);
        let interval = Interval::closed_with_dates(start, NaiveDate::from_ymd(2022, 12, 31));

        assert_eq!(
            interval.start_opt().unwrap(),
            NaiveDate::from_ymd(2022, 1, 1)
        );
        assert_eq!(
            interval.end_opt().unwrap(),
            NaiveDate::from_ymd(2022, 12, 31)
        );

        let duration = interval.duration().unwrap();

        let interval_duration = Interval::closed_from_start(start, duration);
        assert_eq!(interval.start_opt(), interval_duration.start_opt());
        assert_eq!(interval.end_opt(), interval_duration.end_opt());
    }

    #[test]
    fn test_interval_closed_from_start() {
        let mut iter = Interval::closed_from_start(
            NaiveDate::from_ymd(2022, 1, 1),
            RelativeDuration::months(1),
        )
        .until_after(NaiveDate::from_ymd(2023, 1, 1))
        .unwrap();

        let next = iter.next().unwrap();
        assert_eq!(next.start_opt().unwrap(), NaiveDate::from_ymd(2022, 1, 1));
        assert_eq!(next.end_opt().unwrap(), NaiveDate::from_ymd(2022, 2, 1));

        let next = iter.next().unwrap();
        assert_eq!(next.start_opt().unwrap(), NaiveDate::from_ymd(2022, 2, 1));
        assert_eq!(next.end_opt().unwrap(), NaiveDate::from_ymd(2022, 3, 1));

        let next = iter.next().unwrap();
        assert_eq!(next.start_opt().unwrap(), NaiveDate::from_ymd(2022, 3, 1));
    }

    #[test]
    fn test_interval_closed_from_end() {
        let interval = Interval::closed_from_end(
            NaiveDate::from_ymd(2022, 1, 1),
            RelativeDuration::months(1).with_weeks(-2).with_days(2),
        );

        assert_eq!(
            interval.start_opt().unwrap(),
            NaiveDate::from_ymd(2021, 12, 13)
        );
        assert_eq!(interval.end_opt().unwrap(), NaiveDate::from_ymd(2022, 1, 1));
    }

    #[test]
    fn test_interval_closed_with_dates() {
        let mut iter = Interval::closed_with_dates(
            NaiveDate::from_ymd(2022, 1, 1),
            NaiveDate::from_ymd(2023, 1, 1),
        )
        .until_after(NaiveDate::from_ymd(2025, 1, 1))
        .unwrap();

        assert_eq!(
            iter.next().unwrap().start_opt(),
            Some(NaiveDate::from_ymd(2022, 1, 1))
        );
        assert_eq!(
            iter.next().unwrap().start_opt(),
            Some(NaiveDate::from_ymd(2023, 1, 1))
        );
    }
}
