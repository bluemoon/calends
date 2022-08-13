use chrono::NaiveDate;

use crate::{Interval, RelativeDuration};

pub struct ForwardIter {
    date: NaiveDate,
    duration: RelativeDuration,
}

impl ForwardIter {
    pub fn new(date: NaiveDate, duration: RelativeDuration) -> Self {
        Self { date, duration }
    }
}

impl Iterator for ForwardIter {
    type Item = Interval;

    fn next(&mut self) -> Option<Self::Item> {
        let interval = Interval::Closed(self.date, self.duration);
        self.date = self.date + self.duration;
        Some(interval)
    }
}

pub struct BackwardIter {
    date: NaiveDate,
    duration: RelativeDuration,
}

impl BackwardIter {
    pub fn new(date: NaiveDate, duration: RelativeDuration) -> Self {
        Self { date, duration }
    }
}

impl Iterator for BackwardIter {
    type Item = Interval;

    fn next(&mut self) -> Option<Self::Item> {
        let interval = Interval::Closed(self.date, self.duration);
        self.date = self.date + -self.duration;
        Some(interval)
    }
}
