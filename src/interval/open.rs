use chrono::NaiveDate;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

use crate::IntervalLike;

use super::{
    bound::Bound,
    marker,
    parse::{parse_open_end_interval, parse_open_start_interval},
};

/// Indicating that the preceeding direction is unbounded, this is the time leading up to the
/// current time.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpenStartInterval {
    end: NaiveDate,
}

impl OpenStartInterval {
    pub fn new(end: NaiveDate) -> Self {
        Self { end }
    }
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
        serializer.serialize_str(&self.iso8601())
    }
}

pub struct IntervalVisitor;

impl<'de> de::Visitor<'de> for IntervalVisitor {
    type Value = OpenStartInterval;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a ISO8601-2:2019 duration")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        parse_open_start_interval(v.as_bytes())
            .map(|(_, d)| d)
            .map_err(E::custom)
    }
}

impl<'de> Deserialize<'de> for OpenStartInterval {
    fn deserialize<D>(deserializer: D) -> Result<OpenStartInterval, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(IntervalVisitor)
    }
}

/// Indicating that the following direction is unbounded, this is the time after the
/// current time.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpenEndInterval {
    start: NaiveDate,
}

impl OpenEndInterval {
    pub fn new(start: NaiveDate) -> Self {
        Self { start }
    }
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
        serializer.serialize_str(&self.iso8601())
    }
}

pub struct UnboundedEndVisitor;

impl<'de> de::Visitor<'de> for UnboundedEndVisitor {
    type Value = OpenEndInterval;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a ISO8601-2:2019 duration")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        parse_open_end_interval(v.as_bytes())
            .map(|(_, d)| d)
            .map_err(E::custom)
    }
}

impl<'de> Deserialize<'de> for OpenEndInterval {
    fn deserialize<D>(deserializer: D) -> Result<OpenEndInterval, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(UnboundedEndVisitor)
    }
}
