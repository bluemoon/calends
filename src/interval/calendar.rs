use super::{base::Interval, bound};
use crate::{shift, util};
use chrono::NaiveDate;
use fnv::FnvHasher;
use serde::{ser::SerializeStruct, Serialize, Serializer};
use std::{
    hash::{Hash, Hasher},
    ops::Bound,
};

// I want to pay quarterly starting on 2/1
// - What day does this happen on in Q2
//
// I want to pay quarterly starting on 3/31
// - Does this pin to end of quarter?
//
// I want to pay monthly starting 1/31
// - Should this be pinned to end of the e.g. 2/28, 3/31
//
// I want to pay monthly starting 2/28
// - Should this interval produce a date of 3/31 the next time? this will be longer than 30 days
//
// I want to pay yearly
// - What happens in the case of leap years, do we just pin it to the end of Feb or does this go to
// the next day? 3/1

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CalendarBasis {
    Year,
    Quarter,
    Month,
    BiWeek,
    Week,
    Day,
}

/// An interval that is constructed off of the idea of the standard calendar (Modern).
///
/// This interval is by default inclusive on both ends.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Deserialize)]
pub struct CalendarInterval {
    start: NaiveDate,
    end: NaiveDate,
    period: CalendarBasis,
}

impl Serialize for CalendarInterval {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("CalendarInterval", 4)?;
        state.serialize_field("start", &self.start)?;
        state.serialize_field("end", &self.end)?;
        state.serialize_field("period", &self.period)?;
        state.serialize_field("hash", &self.hash_str())?;
        state.end()
    }
}

impl CalendarInterval {
    pub fn from_calendar_basis(basis: CalendarBasis, date: NaiveDate) -> Self {
        match basis {
            CalendarBasis::Year => CalendarInterval::year_for_date(date),
            CalendarBasis::Quarter => CalendarInterval::quarter_for_date(date),
            CalendarBasis::Month => CalendarInterval::month_for_date(date),
            CalendarBasis::BiWeek => CalendarInterval::biweek_for_date(date),
            CalendarBasis::Week => CalendarInterval::week_for_date(date),
            CalendarBasis::Day => CalendarInterval::day_for_date(date),
        }
    }

    /// Convert a date into its corresponding year interval
    ///
    /// This is done by finding the lower and upper bounds of the [CalendarInterval]. A year will
    /// fall with the start of January 1st and end with December 31st
    pub fn year_for_date(date: NaiveDate) -> Self {
        CalendarInterval {
            start: util::beginning_of_year(&date),
            end: util::end_of_year(&date),
            period: CalendarBasis::Year,
        }
    }

    /// Convert a date into its corresponding quarter interval
    ///
    /// This is done by finding the lower and upper bounds of the [CalendarInterval]. A quarter will
    /// fall on January 1, April 1, July 1, or October 1 of the current year.
    pub fn quarter_for_date(date: NaiveDate) -> Self {
        CalendarInterval {
            start: util::beginning_of_quarter(&date),
            end: util::end_of_quarter(&date),
            period: CalendarBasis::Quarter,
        }
    }

    /// Convert a date into its corresponding month interval
    ///
    /// This is done by finding the lower and upper bounds of the [CalendarInterval].
    pub fn month_for_date(date: NaiveDate) -> Self {
        CalendarInterval {
            start: util::beginning_of_month(&date),
            end: util::end_of_month(&date),
            period: CalendarBasis::Month,
        }
    }

    /// Convert a date into its corresponding biweekly interval
    ///
    /// This is done by finding the lower and upper bounds of the [CalendarInterval].
    pub fn biweek_for_date(date: NaiveDate) -> Self {
        CalendarInterval {
            start: util::beginning_of_biweek(&date),
            end: util::end_of_biweek(&date),
            period: CalendarBasis::BiWeek,
        }
    }

    /// Convert a date into its corresponding week interval
    ///
    /// This is done by finding the lower and upper bounds of the [CalendarInterval].
    pub fn week_for_date(date: NaiveDate) -> Self {
        CalendarInterval {
            start: util::beginning_of_week(&date),
            end: util::end_of_week(&date),
            period: CalendarBasis::Week,
        }
    }

    /// Convert a date into its corresponding day interval
    ///
    /// This is done by finding the lower and upper bounds of the [CalendarInterval].
    ///
    /// N.B. this is considered to be a degenerate interval and only includes one item in the set
    pub fn day_for_date(date: NaiveDate) -> Self {
        CalendarInterval {
            start: date,
            end: date,
            period: CalendarBasis::Day,
        }
    }

    /// Returns the successive interval given the current interval
    ///
    pub fn succ(&self) -> Self {
        match self.period {
            CalendarBasis::Year => {
                let start = self.end_date().unwrap().succ();
                let end = shift::add_year_duration(start);

                CalendarInterval {
                    start,
                    end,
                    period: CalendarBasis::Year,
                }
            }
            CalendarBasis::Quarter => {
                let start = self.end_date().unwrap().succ();
                let end = shift::shift_quarters(start, 1);

                CalendarInterval {
                    start,
                    end,
                    period: CalendarBasis::Quarter,
                }
            }
            CalendarBasis::Month => {
                // End of the last month + 1 day
                let start = self.end_date().unwrap().succ();
                let end = util::end_of_month(&start);

                CalendarInterval {
                    start,
                    end,
                    period: CalendarBasis::Month,
                }
            }
            CalendarBasis::BiWeek => {
                let start = self.end_date().unwrap().succ();
                let end = shift::add_biweek_duration(start);

                CalendarInterval {
                    start,
                    end,
                    period: CalendarBasis::BiWeek,
                }
            }
            CalendarBasis::Week => {
                let start = self.end_date().unwrap().succ();
                let end = shift::add_week_duration(start);

                CalendarInterval {
                    start,
                    end,
                    period: CalendarBasis::Week,
                }
            }
            CalendarBasis::Day => {
                let start = self.end_date().unwrap().succ();
                let end = shift::add_day(start);

                CalendarInterval {
                    start: end,
                    end,
                    period: CalendarBasis::Day,
                }
            }
        }
    }

    /// Produce a hash for the interval
    ///
    /// Why do you use FNV?
    ///
    /// Currently there's no guarantee that rust upgrades won't change how hashing functions so we
    /// must consider this as its externally facing.
    pub fn hash_str(&self) -> String {
        let mut hash = FnvHasher::default();
        self.hash(&mut hash);
        base64::encode(hash.finish().to_be_bytes())
    }

    /// Combinator to allow us to iterate until something happens
    pub fn until(&self, date: NaiveDate) -> CalendarUntil {
        CalendarUntil::new(date, self.clone().into_iter())
    }
}

/// An iterator that works off of [CalendarInterval]
///
/// Iterates until a certain point in time
#[derive(Debug, Clone)]
pub struct CalendarUntil {
    pub until: Bound<NaiveDate>,
    pub calendar_iter: CalendarIntoIter,
}

impl CalendarUntil {
    pub fn new(until: NaiveDate, calendar_iter: CalendarIntoIter) -> Self {
        Self {
            until: Bound::Included(until),
            calendar_iter,
        }
    }
}

impl Iterator for CalendarUntil {
    type Item = CalendarInterval;

    fn next(&mut self) -> Option<Self::Item> {
        let event = self.calendar_iter.next()?;
        match bound::cmp_bound(&event.end(), &self.until) {
            std::cmp::Ordering::Less => Some(event),
            std::cmp::Ordering::Equal => Some(event),
            std::cmp::Ordering::Greater => None,
        }
    }
}

impl Interval for CalendarInterval {
    fn start(&self) -> Bound<NaiveDate> {
        Bound::Included(self.start)
    }

    fn end(&self) -> Bound<NaiveDate> {
        Bound::Included(self.end)
    }
}

#[derive(Debug, Clone)]
pub struct CalendarIntoIter {
    interval: CalendarInterval,
}

impl IntoIterator for CalendarInterval {
    type Item = CalendarInterval;
    type IntoIter = CalendarIntoIter;

    fn into_iter(self) -> Self::IntoIter {
        CalendarIntoIter { interval: self }
    }
}

impl Iterator for CalendarIntoIter {
    type Item = CalendarInterval;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.interval.clone();
        self.interval = self.interval.succ();
        Some(current)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_year_for_date() {
        let c1 = CalendarInterval::year_for_date(NaiveDate::from_ymd(2022, 2, 3));
        assert_eq!(c1.start_date(), Some(NaiveDate::from_ymd(2022, 1, 1)));
        assert_eq!(c1.end_date(), Some(NaiveDate::from_ymd(2022, 12, 31)))
    }

    #[ignore = "broken af"]
    #[test]
    fn test_quarter_for_date() {
        let c1 = CalendarInterval::quarter_for_date(NaiveDate::from_ymd(2022, 2, 3));
        assert_eq!(c1.start_date(), Some(NaiveDate::from_ymd(2022, 1, 1)));
        assert_eq!(c1.end_date(), Some(NaiveDate::from_ymd(2022, 3, 31)))
    }

    #[test]
    fn test_month_for_date() {
        let c1 = CalendarInterval::month_for_date(NaiveDate::from_ymd(2022, 2, 3));
        assert_eq!(c1.start_date(), Some(NaiveDate::from_ymd(2022, 2, 1)));
        assert_eq!(c1.end_date(), Some(NaiveDate::from_ymd(2022, 2, 28)))
    }

    #[test]
    fn test_biweek_for_date() {
        let c1 = CalendarInterval::biweek_for_date(NaiveDate::from_ymd(2022, 1, 1));
        assert_eq!(c1.start_date(), Some(NaiveDate::from_ymd(2021, 12, 20)));
        assert_eq!(c1.end_date(), Some(NaiveDate::from_ymd(2022, 1, 2)))
    }

    #[test]
    fn test_year_iterator() {
        let c1 = CalendarInterval::year_for_date(NaiveDate::from_ymd(2022, 2, 3));
        let mut iter = c1.into_iter();

        let n1 = iter.next().unwrap();
        assert_eq!(n1.start_date(), Some(NaiveDate::from_ymd(2022, 1, 1)));

        let n1 = iter.next().unwrap();
        assert_eq!(n1.start_date(), Some(NaiveDate::from_ymd(2023, 1, 1)))
    }

    #[ignore = "broken af"]
    #[test]
    fn test_quarter_iterator() {
        let c1 = CalendarInterval::quarter_for_date(NaiveDate::from_ymd(2022, 2, 3));
        let mut iter = c1.into_iter();

        let n = iter.next().unwrap();
        assert_eq!(n.start_date(), Some(NaiveDate::from_ymd(2022, 1, 1)));
        assert_eq!(n.end_date(), Some(NaiveDate::from_ymd(2022, 3, 31)));

        let n = iter.next().unwrap();
        assert_eq!(n.start_date(), Some(NaiveDate::from_ymd(2022, 4, 1)));
        assert_eq!(n.end_date(), Some(NaiveDate::from_ymd(2022, 7, 1)));
    }

    #[test]
    fn test_month_iterator() {
        let c1 = CalendarInterval::month_for_date(NaiveDate::from_ymd(2022, 1, 1));
        let mut iter = c1.into_iter();

        let n = iter.next().unwrap();
        assert_eq!(n.start_date(), Some(NaiveDate::from_ymd(2022, 1, 1)));
        assert_eq!(n.end_date(), Some(NaiveDate::from_ymd(2022, 1, 31)));

        let n = iter.next().unwrap();
        assert_eq!(n.start_date(), Some(NaiveDate::from_ymd(2022, 2, 1)));
        assert_eq!(n.end_date(), Some(NaiveDate::from_ymd(2022, 2, 28)));

        let n = iter.next().unwrap();
        assert_eq!(n.start_date(), Some(NaiveDate::from_ymd(2022, 3, 1)));
        assert_eq!(n.end_date(), Some(NaiveDate::from_ymd(2022, 3, 31)));
    }

    #[test]
    fn test_biweek_iterator() {
        let c1 = CalendarInterval::biweek_for_date(NaiveDate::from_ymd(2022, 2, 3));
        let mut iter = c1.into_iter();

        let n = iter.next().unwrap();
        assert_eq!(n.start_date(), Some(NaiveDate::from_ymd(2022, 1, 31)));
        assert_eq!(n.end_date(), Some(NaiveDate::from_ymd(2022, 2, 13)));

        let n = iter.next().unwrap();
        assert_eq!(n.start_date(), Some(NaiveDate::from_ymd(2022, 2, 14)));
        assert_eq!(n.end_date(), Some(NaiveDate::from_ymd(2022, 2, 28)));
    }

    #[test]
    fn test_week_iterator() {
        let c1 = CalendarInterval::week_for_date(NaiveDate::from_ymd(2022, 2, 3));
        let mut iter = c1.into_iter();

        let n = iter.next().unwrap();
        assert_eq!(n.start_date(), Some(NaiveDate::from_ymd(2022, 1, 31)));
        assert_eq!(n.end_date(), Some(NaiveDate::from_ymd(2022, 2, 6)));

        let n = iter.next().unwrap();
        assert_eq!(n.start_date(), Some(NaiveDate::from_ymd(2022, 2, 7)));
        assert_eq!(n.end_date(), Some(NaiveDate::from_ymd(2022, 2, 14)));
    }
}
