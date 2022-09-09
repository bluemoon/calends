use crate::IntervalLike;

use super::closed::BoundInterval;
use super::marker;
use super::open::{UnboundedEndInterval, UnboundedStartInterval};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Interval {
    Closed(BoundInterval),
    UnboundedStart(UnboundedStartInterval),
    UnboundedEnd(UnboundedEndInterval),
}

impl IntervalLike for Interval {
    fn bound_start(&self) -> super::bound::Bound<chrono::NaiveDate> {
        match self {
            Interval::Closed(c) => c.bound_start(),
            Interval::UnboundedStart(us) => us.bound_start(),
            Interval::UnboundedEnd(ue) => ue.bound_start(),
        }
    }

    fn bound_end(&self) -> super::bound::Bound<chrono::NaiveDate> {
        match self {
            Interval::Closed(c) => c.bound_end(),
            Interval::UnboundedStart(us) => us.bound_end(),
            Interval::UnboundedEnd(ue) => ue.bound_end(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IntervalWithStart {
    Closed(BoundInterval),
    UnboundedEnd(UnboundedEndInterval),
}

impl IntervalLike for IntervalWithStart {
    fn bound_start(&self) -> super::bound::Bound<chrono::NaiveDate> {
        match self {
            IntervalWithStart::Closed(c) => c.bound_start(),
            IntervalWithStart::UnboundedEnd(ue) => ue.bound_start(),
        }
    }

    fn bound_end(&self) -> super::bound::Bound<chrono::NaiveDate> {
        match self {
            IntervalWithStart::Closed(c) => c.bound_end(),
            IntervalWithStart::UnboundedEnd(ue) => ue.bound_end(),
        }
    }
}

impl marker::Start for IntervalWithStart {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IntervalWithEnd {
    Closed(BoundInterval),
    UnboundedStart(UnboundedEndInterval),
}

impl IntervalLike for IntervalWithEnd {
    fn bound_start(&self) -> super::bound::Bound<chrono::NaiveDate> {
        match self {
            IntervalWithEnd::Closed(c) => c.bound_start(),
            IntervalWithEnd::UnboundedStart(u) => u.bound_start(),
        }
    }

    fn bound_end(&self) -> super::bound::Bound<chrono::NaiveDate> {
        match self {
            IntervalWithEnd::Closed(c) => c.bound_end(),
            IntervalWithEnd::UnboundedStart(u) => u.bound_end(),
        }
    }
}

impl marker::End for IntervalWithEnd {}
