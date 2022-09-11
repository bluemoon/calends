use chrono::NaiveDate;

use super::{marker::End, ClosedInterval};

#[derive(Debug, Clone)]
pub struct UntilAfter<T>
where
    T: Iterator<Item = ClosedInterval>,
{
    iter: T,
    until: NaiveDate,
}

impl<T> UntilAfter<T>
where
    T: Iterator<Item = ClosedInterval>,
{
    pub fn new(iter: T, until: NaiveDate) -> Self {
        UntilAfter { iter, until }
    }
}

impl<T> Iterator for UntilAfter<T>
where
    T: Iterator<Item = ClosedInterval>,
{
    type Item = ClosedInterval;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(item) => {
                if item.end() >= self.until {
                    None
                } else {
                    Some(item)
                }
            }
            None => None,
        }
    }
}
