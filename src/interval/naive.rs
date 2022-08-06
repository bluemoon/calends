use std::{fmt::Display, iter, ops::Bound};

use chrono::NaiveDate;
use serde::{ser::SerializeStruct, Serialize, Serializer};

use crate::bound;

use super::base::BaseInterval;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct NaiveInterval {
  pub start: Bound<NaiveDate>,
  pub end: Bound<NaiveDate>,
}

impl NaiveInterval {
  pub fn new(start: Bound<NaiveDate>, end: Bound<NaiveDate>) -> Self {
    Self { start, end }
  }
}

impl Serialize for NaiveInterval {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let mut start = serializer.serialize_struct("NaiveInterval", 4)?;
    start.serialize_field("start_bound", &bound::to_str(self.start))?;
    start.serialize_field("start_date", &bound::to_opt(self.start))?;
    start.serialize_field("end_bound", &bound::to_str(self.end))?;
    start.serialize_field("end_date", &bound::to_opt(self.end))?;
    start.end()
  }
}

impl Ord for NaiveInterval {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    bound::cmp_range((&self.start(), &self.end()), (&other.start(), &other.end()))
  }
}

impl PartialOrd for NaiveInterval {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    Some(bound::cmp_range(
      (&self.start(), &self.end()),
      (&other.start(), &other.end()),
    ))
  }
}

impl Display for NaiveInterval {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_fmt(format_args!("{:?} to {:?}", self.start, self.end))
  }
}

impl BaseInterval for NaiveInterval {
  fn start(&self) -> Bound<NaiveDate> {
    self.start
  }

  fn end(&self) -> Bound<NaiveDate> {
    self.end
  }
}

pub struct NaiveBasisIntoIter {
  iter: Box<dyn Iterator<Item = NaiveInterval>>,
}

impl IntoIterator for NaiveInterval {
  type Item = NaiveInterval;

  type IntoIter = NaiveBasisIntoIter;

  fn into_iter(self) -> Self::IntoIter {
    let iter = iter::once(self);

    NaiveBasisIntoIter {
      iter: Box::new(iter),
    }
  }
}

impl Iterator for NaiveBasisIntoIter {
  type Item = NaiveInterval;

  fn next(&mut self) -> Option<Self::Item> {
    self.iter.next()
  }
}

#[cfg(test)]
mod tests {
  use std::cmp::Ordering;

  use super::*;

  #[test]
  fn test_ord_eq() {
    let n1 = NaiveInterval {
      start: Bound::Included(NaiveDate::from_ymd(2022, 1, 1)),
      end: Bound::Included(NaiveDate::from_ymd(2022, 5, 18)),
    };
    let n2 = NaiveInterval {
      start: Bound::Included(NaiveDate::from_ymd(2022, 1, 1)),
      end: Bound::Included(NaiveDate::from_ymd(2022, 5, 18)),
    };

    assert_eq!(n1.cmp(&n2), Ordering::Equal)
  }

  #[test]
  fn test_partial_ord_eq() {
    let n1 = NaiveInterval {
      start: Bound::Included(NaiveDate::from_ymd(2022, 1, 1)),
      end: Bound::Included(NaiveDate::from_ymd(2022, 5, 18)),
    };
    let n2 = NaiveInterval {
      start: Bound::Included(NaiveDate::from_ymd(2022, 1, 1)),
      end: Bound::Included(NaiveDate::from_ymd(2022, 12, 31)),
    };

    assert_eq!(n1.partial_cmp(&n2), Some(Ordering::Less))
  }

  #[test]
  fn test_debug() {
    let n1 = NaiveInterval {
      start: Bound::Included(NaiveDate::from_ymd(2022, 1, 1)),
      end: Bound::Included(NaiveDate::from_ymd(2022, 5, 18)),
    };

    assert_eq!(
      format!("{:?}", n1),
      "NaiveInterval { start: Included(2022-01-01), end: Included(2022-05-18) }".to_string()
    )
  }

  #[test]
  fn test_display() {
    let n1 = NaiveInterval {
      start: Bound::Included(NaiveDate::from_ymd(2022, 1, 1)),
      end: Bound::Included(NaiveDate::from_ymd(2022, 5, 18)),
    };

    assert_eq!(
      format!("{}", n1),
      "Included(2022-01-01) to Included(2022-05-18)".to_string()
    )
  }
}
