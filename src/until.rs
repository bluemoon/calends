use std::ops::Bound;

use chrono::NaiveDate;

use crate::interval::bound;

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
