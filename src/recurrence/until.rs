use std::{cmp::Ordering, ops::Bound};

use chrono::NaiveDate;

fn cmp_bound<Q>(e1: &Bound<Q>, e2: &Bound<Q>) -> Ordering
where
    Q: Ord,
{
    // Based on the encoding idea used in `cmp`.
    // Note that we have inversed the 2nd value in the tuple,
    // as the Included/Excluded rules are flipped for the upper bound.
    let e1 = match e1 {
        Bound::Included(x) => Some((x, 2)),
        Bound::Excluded(x) => Some((x, 1)),
        Bound::Unbounded => None,
    };
    let e2 = match e2 {
        Bound::Included(x) => Some((x, 2)),
        Bound::Excluded(x) => Some((x, 1)),
        Bound::Unbounded => None,
    };

    match (e1, e2) {
        (None, None) => Ordering::Equal,
        (None, Some(_)) => Ordering::Greater,
        (Some(_), None) => Ordering::Less,
        (Some(r1), Some(ref r2)) => r1.cmp(r2),
    }
}

/// Iterates until a certain point in time
#[derive(Debug, Clone)]
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
        match cmp_bound(&Bound::Included(event), &self.until) {
            std::cmp::Ordering::Less => Some(event),
            std::cmp::Ordering::Equal => Some(event),
            std::cmp::Ordering::Greater => None,
        }
    }
}
