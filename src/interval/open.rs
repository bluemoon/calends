use chrono::NaiveDate;
use serde::{Serialize, Serializer};

use crate::IntervalLike;

use super::{bound::Bound, marker, serde::SerializeInterval};

/// Indicating that the preceeding direction is unbounded, this is the time leading up to the
/// current time.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnboundedStartInterval {
    end: NaiveDate,
}

impl UnboundedStartInterval {
    pub fn new(end: NaiveDate) -> Self {
        Self { end }
    }
}

impl IntervalLike for UnboundedStartInterval {
    fn bound_start(&self) -> Bound<NaiveDate> {
        Bound::Unbounded
    }

    fn bound_end(&self) -> Bound<NaiveDate> {
        Bound::Included(self.end)
    }
}

impl marker::End for UnboundedStartInterval {}

impl Serialize for UnboundedStartInterval {
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
pub struct UnboundedEndInterval {
    start: NaiveDate,
}

impl UnboundedEndInterval {
    pub fn new(start: NaiveDate) -> Self {
        Self { start }
    }
}

impl IntervalLike for UnboundedEndInterval {
    fn bound_start(&self) -> Bound<NaiveDate> {
        Bound::Included(self.start)
    }

    fn bound_end(&self) -> Bound<NaiveDate> {
        Bound::Unbounded
    }
}

impl marker::Start for UnboundedEndInterval {}

impl Serialize for UnboundedEndInterval {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        SerializeInterval(self.clone()).serialize(serializer)
    }
}
