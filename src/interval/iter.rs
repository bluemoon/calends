use chrono::NaiveDate;

use crate::{Interval, RelativeDuration};

pub struct Iter {
    date: NaiveDate,
    duration: RelativeDuration,
}

impl Iter {
    pub fn new(date: NaiveDate, duration: RelativeDuration) -> Self {
        Self { date, duration }
    }

    pub fn until_after(self, until: NaiveDate) -> UntilAfter<Iter> {
        UntilAfter::new(self, until)
    }
}

impl Iterator for Iter {
    type Item = Interval;

    fn next(&mut self) -> Option<Self::Item> {
        let interval = Interval::Closed(self.date, self.duration);
        self.date = self.date + self.duration;
        Some(interval)
    }
}

pub struct UntilAfter<T>
where
    T: Iterator<Item = Interval>,
{
    iter: T,
    until: NaiveDate,
}

impl<T> UntilAfter<T>
where
    T: Iterator<Item = Interval>,
{
    pub fn new(iter: T, until: NaiveDate) -> Self {
        UntilAfter { iter, until }
    }
}

impl<T> Iterator for UntilAfter<T>
where
    T: Iterator<Item = Interval>,
{
    type Item = Interval;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(item) => {
                // SAFETY: only bounded intervals have an iterator so unsafe is safe to call on it
                if item.end_date().unwrap() >= self.until {
                    None
                } else {
                    Some(item)
                }
            }
            None => None,
        }
    }
}
