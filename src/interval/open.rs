use chrono::NaiveDate;
use serde::{Serialize, Serializer};

use crate::IntervalLike;

use super::{bound::Bound, marker, serde::SerializeInterval};

/// Indicating that the preceeding direction is unbounded, this is the time leading up to the
/// current time.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpenStartInterval {
    end: NaiveDate,
}

impl IntervalLike for OpenStartInterval {
    fn bound_start(&self) -> Bound<NaiveDate> {
        Bound::Unbounded
    }

    fn bound_end(&self) -> Bound<NaiveDate> {
        Bound::Included(self.end)
    }
}

impl marker::End for OpenStartInterval {}

impl Serialize for OpenStartInterval {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        SerializeInterval(self.clone()).serialize(serializer)
    }
}

/// Indicating that the following direction is unbounded, this is the time after the
/// current time.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpenEndInterval {
    start: NaiveDate,
}

impl IntervalLike for OpenEndInterval {
    fn bound_start(&self) -> Bound<NaiveDate> {
        Bound::Included(self.start)
    }

    fn bound_end(&self) -> Bound<NaiveDate> {
        Bound::Unbounded
    }
}

impl marker::Start for OpenEndInterval {}

impl Serialize for OpenEndInterval {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        SerializeInterval(self.clone()).serialize(serializer)
    }
}
