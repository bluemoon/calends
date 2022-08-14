use chrono::NaiveDate;

use crate::IntervalLike;

use super::bound::Bound;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum OpenInterval {
    /// Indicating that the preceeding direction is unbounded, this is the time leading up to the
    /// current time.
    Start(NaiveDate),
    /// Indicating that the following direction is unbounded, this is the time after the
    /// current time.
    End(NaiveDate),
}

impl OpenInterval {
    /// ISO8601-2:2019 Formatting of intervals
    ///
    /// The standard allows for:
    ///
    /// ```ignore
    ///
    /// - tiseE =[dtE]["/"][dtE]
    /// - tisdE = [dtE]["/"][duration]
    /// - tisdE = [duration]["/"][dtE]
    ///
    /// ```
    /// Currently we only represent the top one
    ///
    pub fn iso8601(&self) -> String {
        match self {
            OpenInterval::Start(date) => format!("../{}", date.to_string()),
            OpenInterval::End(date) => format!("{}/..", date.to_string()),
        }
    }
}

impl IntervalLike for OpenInterval {
    fn start(&self) -> Bound<NaiveDate> {
        match self {
            OpenInterval::Start(s) => Bound::Included(*s),
            OpenInterval::End(_) => Bound::Unbounded,
        }
    }

    fn end(&self) -> Bound<NaiveDate> {
        match self {
            OpenInterval::Start(e) => Bound::Included(*e),
            OpenInterval::End(_) => Bound::Unbounded,
        }
    }
}
