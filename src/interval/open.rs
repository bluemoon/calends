use chrono::NaiveDate;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum OpenInterval {
    /// Indicating that the preceeding direction is unbounded, this is the time leading up to the
    /// current time.
    OpenStart(NaiveDate),
    /// Indicating that the following direction is unbounded, this is the time after the
    /// current time.
    OpenEnd(NaiveDate),
}

impl OpenInterval {
    /// ISO8601-2:2019 Formatting of intervals
    ///
    /// The standard allows for:
    ///
    /// - tiseE =[dtE]["/"][dtE]
    /// - tisdE = [dtE]["/"][duration]
    /// - tisdE = [duration]["/"][dtE]
    ///
    /// Currently we only represent the top one
    ///
    pub fn iso8601(&self) -> String {
        match self {
            OpenInterval::OpenStart(date) => format!("../{}", date.to_string()),
            OpenInterval::OpenEnd(date) => format!("{}/..", date.to_string()),
        }
    }
}
