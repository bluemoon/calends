use chrono::NaiveDate;

///! # Recurrence
///!
///! ## Rules for recurrence
///!
///! - 1.1: Positive offset within interval: First of the month (any period within [CalendarBasis])
///! - 1.2: Negative offset within interval: Last of the month (any period within [CalendarBasis])
///!
///! - 2.1: Positive occurence within interval w/ specific day of week: first wednesday of the month
///!   - Limited to biweeks, months, quarters and years
///! - 2.2: Negative occurence within interval w/ specific day of week: last wednesday of the month
///!
///! - 3.1: Day of week inside an a week
///
///! ## Combinators on recurrence
///!
///! - Until a point in time (inclusive or exclusive)
///! - Count of recurrences (end after a count of occurences) (inclusive)
use crate::{interval::calendar::CalendarBasis, relative::RelativeDuration, util};

pub enum Rule {
    /// An offset within an interval
    ///
    /// - RelativeDuration: Interval that this occurs in
    /// - Offset: the offset in days with positive starting at the beginning of the cycle and
    /// negative being referenced from the end of the interval.
    ///
    /// This covers cases 1.1 and 1.2 in the rules of recurrence
    Offset(RelativeDuration, i32),

    /// An occurence within an interval
    ///
    /// - Basis ([CalendarBasis]): the Interval that this occurs within
    /// - Offset ([i32]): the offset of this occurence e.g. 3rd wednesday
    /// - Weekday ([chrono::Weekday]): Day of week that this happens on
    ///
    /// This covers cases 2.1 and 2.2
    ///
    /// Note: using a [CalendarBasis] of Day and Week is undefined
    Occurence(CalendarBasis, i32, chrono::Weekday),
}

impl Rule {
    /// Create a recurrence that occurs on a monthly basis
    pub fn monthly() -> Rule {
        Rule::Offset(RelativeDuration::months(1), 0)
    }
}

/// Evaluate an existing rule
pub struct Recurrence {
    pub rule: Rule,
    occurence_count: i32,
    date: NaiveDate,
}

impl Recurrence {
    pub fn with_start(rule: Rule, date: NaiveDate) -> Self {
        let date = match rule {
            Rule::Offset(_, _) => util::start_of_month(date),
            Rule::Occurence(_, _, _) => todo!(),
        };

        Self {
            rule,
            occurence_count: 0,
            date,
        }
    }
}

impl Iterator for Recurrence {
    type Item = NaiveDate;

    fn next(&mut self) -> Option<Self::Item> {
        let date = self.date;

        match &self.rule {
            Rule::Offset(duration, _) => self.date = date + duration.clone(),
            Rule::Occurence(_, _, _) => todo!(),
        }

        Some(date)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recurrence() {
        let date = NaiveDate::from_ymd(2022, 1, 3);

        let mut recur = Recurrence::with_start(Rule::monthly(), date).into_iter();
        assert_eq!(recur.next(), Some(NaiveDate::from_ymd(2022, 1, 1)));
    }
}
