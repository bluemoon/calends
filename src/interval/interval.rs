use crate::duration::RelativeDuration;

use super::base::IntervalLike;
use chrono::NaiveDate;
use fnv::FnvHasher;
use std::{
    hash::{Hash, Hasher},
    ops::Bound,
};

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
///
/// - **End and duration:** The end time minus the duration creates the beginning of the interval.
/// Intervals are then iterated on with the given duration.
///
/// ## Other notes
///
/// - This interval is by default inclusive on both ends.
///
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Interval {
    start: NaiveDate,
    duration: RelativeDuration,
    // TODO: we may want an enumeration to support different serialization types
}

impl Interval {
    /// Create an interval from a start and a duration
    pub fn from_start(start: NaiveDate, duration: RelativeDuration) -> Self {
        Self { start, duration }
    }

    /// Create an interval from an end and a duration
    pub fn from_end(end: NaiveDate, duration: RelativeDuration) -> Self {
        // TODO: subtract for duration
        Self {
            start: end + -duration,
            duration,
        }
    }

    /// Create an interval with a specified set of dates
    pub fn with_dates(start: NaiveDate, _end: NaiveDate) -> Self {
        // TODO: determine the duration based on the delta
        Self {
            start,
            duration: RelativeDuration::months(1),
        }
    }

    /// Produce a hash for the interval
    ///
    /// # Why do you use FNV?
    ///
    /// Currently there's no guarantee that rust upgrades won't change how hashing functions so we
    /// must consider this as its externally facing.
    pub fn hash_str(&self) -> String {
        let mut hash = FnvHasher::default();
        self.hash(&mut hash);
        base64::encode(hash.finish().to_be_bytes())
    }
}

impl IntervalLike for Interval {
    fn start(&self) -> Bound<NaiveDate> {
        Bound::Included(self.start)
    }

    fn end(&self) -> Bound<NaiveDate> {
        Bound::Included(self.start + self.duration)
    }
}

impl Iterator for Interval {
    type Item = Interval;

    fn next(&mut self) -> Option<Self::Item> {
        let interval = self.clone();
        self.start = interval.start + self.duration;

        Some(interval)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interval() {
        let start = NaiveDate::from_ymd(2022, 1, 1);
        let duration = RelativeDuration::months(1);

        let mut interval = Interval::from_start(start, duration);

        assert_eq!(interval.start_date(), Some(start));
        assert_eq!(interval.end_date(), Some(NaiveDate::from_ymd(2022, 2, 1)));

        // Intervals are inclusive so return the current time span first
        let next = interval.next().unwrap();

        assert_eq!(next.start_date(), Some(NaiveDate::from_ymd(2022, 1, 1)));
        assert_eq!(next.end_date(), Some(NaiveDate::from_ymd(2022, 2, 1)));

        let next = interval.next().unwrap();

        assert_eq!(next.start_date(), Some(NaiveDate::from_ymd(2022, 2, 1)));
        assert_eq!(next.end_date(), Some(NaiveDate::from_ymd(2022, 3, 1)));
    }
}
