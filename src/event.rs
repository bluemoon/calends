use chrono::NaiveDate;

use crate::recur::{Recurrence, Rule};

/// An event happens on a date and can be recurring but depends on the basis
#[derive(Debug, Clone, Copy)]
pub struct Event(NaiveDate);

impl Event {
    pub fn new(date: NaiveDate) -> Self {
        Self(date)
    }

    pub fn recur(&self, rule: Rule) -> Recurrence {
        Recurrence::with_start(rule, self.0)
    }
}
