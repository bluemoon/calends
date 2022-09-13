use crate::{duration::RelativeDuration, IntervalLike};

use super::{bound::Bound, iter::UntilAfter, marker, parse::parse_interval};
use chrono::NaiveDate;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

/// An interval that is constructed off of the idea of the standard calendar (Gregorian Proleptic
/// calendar).
///
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ClosedInterval {
    /// Indicating up to OR on in the direction of the interval
    ///
    /// e.g. if the direction is "forwards" and the end is inclusive then it will include the
    /// specified end date
    date: NaiveDate,
    pub(crate) duration: RelativeDuration,
}

impl ClosedInterval {
    /// Create an interval from a start and a duration
    pub fn from_start(date: NaiveDate, duration: RelativeDuration) -> Self {
        ClosedInterval { date, duration }
    }

    /// Create an interval from an end and a duration
    pub fn from_end(end: NaiveDate, duration: RelativeDuration) -> Self {
        ClosedInterval {
            date: end + -duration,
            duration,
        }
    }

    /// Create an interval with a specified set of dates
    pub fn with_dates(start: NaiveDate, end: NaiveDate) -> Self {
        ClosedInterval {
            date: start,
            duration: RelativeDuration::from_duration_between(start, end),
        }
    }

    fn adjust_duration(duration: RelativeDuration) -> RelativeDuration {
        match duration.cmp(&RelativeDuration::zero()) {
            std::cmp::Ordering::Less => duration + RelativeDuration::default().with_days(1),
            std::cmp::Ordering::Equal => duration,
            std::cmp::Ordering::Greater => duration - RelativeDuration::default().with_days(1),
        }
    }

    /// Start date of the interval
    fn computed_start_date(&self) -> NaiveDate {
        self.date
    }

    /// End date of the interval
    fn computed_end_date(&self) -> NaiveDate {
        self.date + ClosedInterval::adjust_duration(self.duration)
    }

    pub fn until_after(self, until: NaiveDate) -> UntilAfter<ClosedInterval> {
        UntilAfter::new(self, until)
    }
}

impl IntervalLike for ClosedInterval {
    fn bound_start(&self) -> Bound<NaiveDate> {
        Bound::Included(self.computed_start_date())
    }

    fn bound_end(&self) -> Bound<NaiveDate> {
        Bound::Included(self.computed_end_date())
    }

    fn duration(&self) -> Option<RelativeDuration> {
        Some(self.duration)
    }
}

impl marker::Start for ClosedInterval {}
impl marker::End for ClosedInterval {}

/// Serialize a `Interval` as a ISO8601-2:2019 compatible format
impl Serialize for ClosedInterval {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.iso8601())
    }
}

pub struct IntervalVisitor;

impl<'de> de::Visitor<'de> for IntervalVisitor {
    type Value = ClosedInterval;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a ISO8601-2:2019 duration")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        parse_interval(v.as_bytes())
            .map(|(_, d)| d)
            .map_err(E::custom)
    }
}

impl<'de> Deserialize<'de> for ClosedInterval {
    fn deserialize<D>(deserializer: D) -> Result<ClosedInterval, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_string(IntervalVisitor)
    }
}

impl Iterator for ClosedInterval {
    type Item = ClosedInterval;

    fn next(&mut self) -> Option<Self::Item> {
        let interval = ClosedInterval::from_start(self.date, self.duration);
        // to prevent overlapping dates we add one day
        self.date = self.date + self.duration;
        Some(interval)
    }
}
