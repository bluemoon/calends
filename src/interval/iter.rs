use chrono::NaiveDate;

use super::{marker::End, BoundInterval};

#[derive(Debug, Clone)]
pub struct UntilAfter<T>
where
    T: Iterator<Item = BoundInterval>,
{
    iter: T,
    until: NaiveDate,
}

impl<T> UntilAfter<T>
where
    T: Iterator<Item = BoundInterval>,
{
    pub fn new(iter: T, until: NaiveDate) -> Self {
        UntilAfter { iter, until }
    }
}

impl<T> Iterator for UntilAfter<T>
where
    T: Iterator<Item = BoundInterval>,
{
    type Item = BoundInterval;

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
