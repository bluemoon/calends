///! Interval
///!
///! Used to coalesce both recurring and non-recurring intervals into one interface.
use super::bound::{self, Bound};
use chrono::NaiveDate;

pub trait IntervalLike {
    fn bound_start(&self) -> Bound<NaiveDate>;
    fn bound_end(&self) -> Bound<NaiveDate>;

    /// Start date in the form of an option
    ///
    /// If the bounds of the interval for the start date is Unbounded then this function will give
    /// us[None]
    fn start_opt(&self) -> Option<NaiveDate> {
        match self.bound_start() {
            Bound::Included(d) => Some(d),
            Bound::Unbounded => None,
        }
    }

    /// End date in the form of an option
    ///
    /// Unbounded gives us [None]
    fn end_opt(&self) -> Option<NaiveDate> {
        match self.bound_end() {
            Bound::Included(d) => Some(d),
            Bound::Unbounded => None,
        }
    }

    /// Determine whether a date falls within the current interval
    ///
    fn within(&self, date: NaiveDate) -> bool {
        bound::within(date, &self.bound_start(), &self.bound_end())
    }

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
    fn iso8601(&self) -> String {
        match (self.bound_start(), self.bound_end()) {
            (Bound::Included(s), Bound::Included(e)) => format!("{}/{}", s, e),
            (Bound::Included(s), Bound::Unbounded) => format!("{}/..", s),
            (Bound::Unbounded, Bound::Included(e)) => format!("../{}", e),
            // yeah don't unbound it on both sides because thats just weird
            // but we still represent it
            (Bound::Unbounded, Bound::Unbounded) => "../..".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, PartialEq, Eq, serde::Serialize)]
    struct Int {
        pub start: NaiveDate,
        pub end: NaiveDate,
    }

    impl IntervalLike for Int {
        fn bound_start(&self) -> Bound<NaiveDate> {
            Bound::Included(self.start)
        }

        fn bound_end(&self) -> Bound<NaiveDate> {
            Bound::Included(self.end)
        }
    }

    #[test]
    fn test_within() {
        let i1 = Int {
            start: NaiveDate::from_ymd(2022, 1, 1),
            end: NaiveDate::from_ymd(2022, 12, 31),
        };

        assert!(i1.within(NaiveDate::from_ymd(2022, 5, 18)));
        assert!(!i1.within(NaiveDate::from_ymd(2023, 5, 18)));
    }

    #[test]
    fn test_start_date() {
        let i1 = Int {
            start: NaiveDate::from_ymd(2022, 1, 1),
            end: NaiveDate::from_ymd(2022, 12, 31),
        };

        assert_eq!(i1.start_opt(), NaiveDate::from_ymd_opt(2022, 1, 1));
    }

    #[test]
    fn test_end_date() {
        let i1 = Int {
            start: NaiveDate::from_ymd(2022, 1, 1),
            end: NaiveDate::from_ymd(2022, 12, 31),
        };

        assert_eq!(i1.end_opt(), NaiveDate::from_ymd_opt(2022, 12, 31));
    }
}
