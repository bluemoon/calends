use std::ops::Bound;

use crate::{interval::bound, relative::RelativeDuration};
use chrono::NaiveDate;

/// Structure for how an interval of time gets repeated
///
/// ## Rules for recurrence
///
/// - **1.1**: Positive offset within interval e.g. First of the month
/// - **1.2**: Negative offset within interval e.g. Last of the month
///
/// - **2.1**: Positive occurence within interval w/ specific day of week: first wednesday of the month
///   - Limited to biweeks, months, quarters and years
/// - **2.2**: Negative occurence within interval w/ specific day of week: last wednesday of the month
///
/// - **3.1**: Day of week inside an a week
///
/// ## Combinators on recurrence
///
/// - Until a point in time (inclusive or exclusive)
/// - Count of recurrences (end after a count of occurences) (inclusive)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Rule {
    /// An offset within an interval
    ///
    /// - Duration ([RelativeDuration]): A duration of time
    /// - Offset ([i32]): the offset in days with positive starting at the beginning of the cycle and
    /// negative being referenced from the end of the interval.
    ///
    /// This covers cases 1.1 and 1.2 in the rules of recurrence
    Offset(RelativeDuration, i32),

    /// An occurence within an interval
    ///
    /// - Duration ([RelativeDuration]): the duration of time the event happens in
    /// - Offset ([i32]): the offset of this occurence e.g. 3rd wednesday
    /// - Weekday ([chrono::Weekday]): Day of week that this happens on
    ///
    /// This covers cases 2.1 and 2.2
    ///
    /// Note: using a [CalendarBasis] of Day and Week is undefined
    ///
    /// TODO: Describe the ruleset for finding a day of the week
    Occurence(RelativeDuration, i32, chrono::Weekday),
}

impl Rule {
    /// Create a recurrence that occurs on a quarterly basis
    pub fn quarterly() -> Rule {
        Rule::Offset(RelativeDuration::months(3), 0)
    }

    /// Create a recurrence that occurs on a monthly basis
    pub fn monthly() -> Rule {
        Rule::Offset(RelativeDuration::months(1), 0)
    }

    /// Create a recurrence that occurs on a weekly basis
    pub fn biweekly() -> Rule {
        Rule::Offset(RelativeDuration::weeks(2), 0)
    }

    /// Create a recurrence that occurs on a weekly basis
    pub fn weekly() -> Rule {
        Rule::Offset(RelativeDuration::weeks(1), 0)
    }

    /// Create a recurrence that occurs on a monthly basis
    pub fn daily() -> Rule {
        Rule::Offset(RelativeDuration::days(1), 0)
    }
}

/// Evaluate an existing rule
#[derive(Debug, Clone)]
pub struct Recurrence {
    rule: Rule,
    #[allow(dead_code)]
    occurence_count: i32,
    date: NaiveDate,
}

impl Recurrence {
    /// Starting point for the recurring series
    ///
    /// TODO: add the [Rule::Offset] to the start date
    pub fn with_start(rule: Rule, date: NaiveDate) -> Self {
        Self {
            rule,
            occurence_count: 0,
            date,
        }
    }

    /// Iterate up to a date
    ///
    /// TODO: example
    pub fn until(&self, date: NaiveDate) -> Until<Recurrence> {
        Until::exclusive(date, self.clone().into_iter())
    }

    /// Iterate up to and including the date
    ///
    /// TODO: example
    pub fn until_and_including(&self, date: NaiveDate) -> Until<Recurrence> {
        Until::inclusive(date, self.clone().into_iter())
    }
}

impl Iterator for Recurrence {
    type Item = NaiveDate;

    fn next(&mut self) -> Option<Self::Item> {
        let date = self.date;

        match &self.rule {
            Rule::Offset(duration, _) => {
                self.date = date + duration.clone();
                Some(date)
            }
            Rule::Occurence(duration, count, _) => {
                if count < &self.occurence_count {
                    self.date = date + duration.clone();
                    Some(date)
                } else {
                    None
                }
            }
        }
    }
}

/// Iterates until a certain point in time
#[derive(Debug)]
pub struct Until<T>
where
    T: Iterator<Item = NaiveDate>,
{
    pub until: Bound<NaiveDate>,
    pub iter: T,
}

impl<T> Until<T>
where
    T: Iterator<Item = NaiveDate>,
{
    pub fn inclusive(until: NaiveDate, iter: T) -> Self {
        Self {
            until: Bound::Included(until),
            iter,
        }
    }

    pub fn exclusive(until: NaiveDate, iter: T) -> Self {
        Self {
            until: Bound::Excluded(until),
            iter,
        }
    }
}

impl<T> Iterator for Until<T>
where
    T: Iterator<Item = NaiveDate>,
{
    type Item = NaiveDate;

    fn next(&mut self) -> Option<Self::Item> {
        let event = self.iter.next()?;
        match bound::cmp_bound(&Bound::Included(event), &self.until) {
            std::cmp::Ordering::Less => Some(event),
            std::cmp::Ordering::Equal => Some(event),
            std::cmp::Ordering::Greater => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recur_monthly_until_inclusive() {
        let date = NaiveDate::from_ymd(2022, 1, 1);
        let end = NaiveDate::from_ymd(2022, 3, 1);

        let mut recur = Recurrence::with_start(Rule::monthly(), date).until_and_including(end);
        assert_eq!(recur.next(), Some(NaiveDate::from_ymd(2022, 1, 1)));
        assert_eq!(recur.next(), Some(NaiveDate::from_ymd(2022, 2, 1)));
        assert_eq!(recur.next(), Some(NaiveDate::from_ymd(2022, 3, 1)));
        assert_eq!(recur.next(), None);
    }

    #[test]
    fn test_recur_monthly_until_exclusive() {
        let date = NaiveDate::from_ymd(2022, 1, 1);
        let end = NaiveDate::from_ymd(2022, 3, 1);

        let mut recur = Recurrence::with_start(Rule::monthly(), date).until(end);
        assert_eq!(recur.next(), Some(NaiveDate::from_ymd(2022, 1, 1)));
        assert_eq!(recur.next(), Some(NaiveDate::from_ymd(2022, 2, 1)));
        assert_eq!(recur.next(), None);
    }

    #[test]
    fn test_recur_monthly() {
        let date = NaiveDate::from_ymd(2022, 1, 1);

        let mut recur = Recurrence::with_start(Rule::monthly(), date).into_iter();
        assert_eq!(recur.next(), Some(NaiveDate::from_ymd(2022, 1, 1)));
        assert_eq!(recur.next(), Some(NaiveDate::from_ymd(2022, 2, 1)));
    }

    #[test]
    fn test_recur_quarterly() {
        let date = NaiveDate::from_ymd(2022, 1, 1);

        let mut recur = Recurrence::with_start(Rule::quarterly(), date).into_iter();
        assert_eq!(recur.next(), Some(NaiveDate::from_ymd(2022, 1, 1)));
        assert_eq!(recur.next(), Some(NaiveDate::from_ymd(2022, 4, 1)));
    }
}
