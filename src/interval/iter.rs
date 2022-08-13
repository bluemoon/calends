use chrono::NaiveDate;

use crate::Interval;

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
                if item.end_date() >= self.until {
                    None
                } else {
                    Some(item)
                }
            }
            None => None,
        }
    }
}
