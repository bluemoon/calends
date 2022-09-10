use chrono::NaiveDate;

use crate::{IntervalLike, RelativeDuration};

use super::bound::Bound;
use super::closed::BoundInterval;
use super::marker;
use super::open::{UnboundedEndInterval, UnboundedStartInterval};

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
#[serde(tag = "type")]
pub enum Interval {
    Closed(BoundInterval),
    OpenStart(UnboundedStartInterval),
    OpenEnd(UnboundedEndInterval),
}

impl Interval {
    pub fn closed_from_start(date: NaiveDate, duration: RelativeDuration) -> Self {
        Interval::Closed(BoundInterval::from_start(date, duration))
    }

    pub fn closed_from_end(end: NaiveDate, duration: RelativeDuration) -> Self {
        Interval::Closed(BoundInterval::from_end(end, duration))
    }

    pub fn closed_with_dates(start: NaiveDate, end: NaiveDate) -> Self {
        Interval::Closed(BoundInterval::with_dates(start, end))
    }

    pub fn open_start(end: NaiveDate) -> Self {
        Interval::OpenStart(UnboundedStartInterval::new(end))
    }

    pub fn open_end(start: NaiveDate) -> Self {
        Interval::OpenEnd(UnboundedEndInterval::new(start))
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
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IntervalWithStart {
    Closed(BoundInterval),
    UnboundedEnd(UnboundedEndInterval),
}

impl IntervalLike for IntervalWithStart {
    fn bound_start(&self) -> Bound<chrono::NaiveDate> {
        match self {
            IntervalWithStart::Closed(c) => c.bound_start(),
            IntervalWithStart::UnboundedEnd(ue) => ue.bound_start(),
        }
    }

    fn bound_end(&self) -> Bound<chrono::NaiveDate> {
        match self {
            IntervalWithStart::Closed(c) => c.bound_end(),
            IntervalWithStart::UnboundedEnd(ue) => ue.bound_end(),
        }
    }
}

impl marker::Start for IntervalWithStart {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IntervalWithEnd {
    Closed(BoundInterval),
    UnboundedStart(UnboundedEndInterval),
}

impl IntervalLike for IntervalWithEnd {
    fn bound_start(&self) -> Bound<chrono::NaiveDate> {
        match self {
            IntervalWithEnd::Closed(c) => c.bound_start(),
            IntervalWithEnd::UnboundedStart(u) => u.bound_start(),
        }
    }

    fn bound_end(&self) -> Bound<chrono::NaiveDate> {
        match self {
            IntervalWithEnd::Closed(c) => c.bound_end(),
            IntervalWithEnd::UnboundedStart(u) => u.bound_end(),
        }
    }
}

impl marker::End for IntervalWithEnd {}
