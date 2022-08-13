use crate::duration::RelativeDuration;

use super::iter::{BackwardIter, ForwardIter};
use chrono::NaiveDate;
use fnv::FnvHasher;
use std::{
    fmt,
    hash::{Hash, Hasher},
};

/// Iteration Error
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
/// We use this over [ops::Bound] because bound supports exclusive boundaries and we have made the
/// decision that it adds too much cognitive load / API cruft so we do not include it.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Interval {
    /// Indicating that the preceeding direction is unbounded, this is the time leading up to the
    /// current time.
    PreceedingOpen(NaiveDate),
    /// Indicating that the following direction is unbounded, this is the time after the
    /// current time.
    FollowingOpen(NaiveDate),
    /// Indicating up to OR on in the direction of the interval
    ///
    /// e.g. if the direction is "forwards" and the end is inclusive then it will include the
    /// specified end date
    Closed(NaiveDate, RelativeDuration),
}

impl Interval {
    /// Create an interval from a start and a duration
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
    /// assert_eq!(interval.start_date(), Some(start));
    /// assert_eq!(interval.end_date(), Some(NaiveDate::from_ymd(2022, 2, 1)));
    /// ```
    pub fn from_start(date: NaiveDate, duration: RelativeDuration) -> Self {
        Interval::Closed(date, duration)
    }

    /// Create an interval from an end and a duration
    pub fn from_end(end: NaiveDate, duration: RelativeDuration) -> Self {
        Interval::Closed(end + -duration, duration)
    }

    /// Create an interval with a specified set of dates
    pub fn with_dates(date: NaiveDate, _end: NaiveDate) -> Self {
        // TODO: determine the duration based on the delta
        Interval::Closed(date, RelativeDuration::months(1))
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

    /// ISO8601-2:2019 Formatting of intervals
    pub fn iso8601(&self) -> String {
        match self {
            Interval::PreceedingOpen(date) => todo!("../{}", date.to_string()),
            Interval::FollowingOpen(date) => format!("{}/..", date.to_string()),
            Interval::Closed(date, duration) => {
                let start = date.to_string();
                let end = (*date + *duration).to_string();

                format!("{}/{}", start, end)
            }
        }
    }

    pub fn iter_forward(&self) -> Result<ForwardIter, ImpossibleIterator> {
        match self {
            Interval::PreceedingOpen(_) => Err(ImpossibleIterator),
            Interval::FollowingOpen(_) => Err(ImpossibleIterator),
            Interval::Closed(date, duration) => Ok(ForwardIter::new(*date, *duration)),
        }
    }

    pub fn iter_backward(&self) -> Result<BackwardIter, ImpossibleIterator> {
        match self {
            Interval::PreceedingOpen(_) => Err(ImpossibleIterator),
            Interval::FollowingOpen(_) => Err(ImpossibleIterator),
            Interval::Closed(date, duration) => Ok(BackwardIter::new(*date, *duration)),
        }
    }
}
