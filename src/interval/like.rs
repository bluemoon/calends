///! Interval
///!
///! Used to coalesce both recurring and non-recurring intervals into one interface.
use super::bound::{self, Bound};
use chrono::NaiveDate;

pub trait IntervalLike {
    fn start(&self) -> Bound<NaiveDate>;
    fn end(&self) -> Bound<NaiveDate>;

    /// Start date in the form of an option
    ///
    /// Unbounded gives us [None]
    fn start_date(&self) -> Option<NaiveDate> {
        match self.start() {
            Bound::Included(d) => Some(d),
            Bound::Unbounded => None,
        }
    }

    /// End date in the form of an option
    ///
    /// Unbounded gives us [None]
    fn end_date(&self) -> Option<NaiveDate> {
        match self.end() {
            Bound::Included(d) => Some(d),
            Bound::Unbounded => None,
        }
    }

    /// Determine whether a date falls within the current interval
    ///
    fn within(&self, date: NaiveDate) -> bool {
        bound::within(date, &self.start(), &self.end())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Int {
        pub start: Bound<NaiveDate>,
        pub end: Bound<NaiveDate>,
    }

    impl IntervalLike for Int {
        fn start(&self) -> Bound<NaiveDate> {
            self.start.clone()
        }

        fn end(&self) -> Bound<NaiveDate> {
            self.end.clone()
        }
    }

    #[test]
    fn test_within() {
        let i1 = Int {
            start: Bound::Included(NaiveDate::from_ymd(2022, 1, 1)),
            end: Bound::Included(NaiveDate::from_ymd(2022, 12, 31)),
        };

        assert_eq!(i1.within(NaiveDate::from_ymd(2022, 5, 18)), true);
        assert_eq!(i1.within(NaiveDate::from_ymd(2023, 5, 18)), false);
    }

    #[test]
    fn test_start_date() {
        let i1 = Int {
            start: Bound::Included(NaiveDate::from_ymd(2022, 1, 1)),
            end: Bound::Included(NaiveDate::from_ymd(2022, 12, 31)),
        };

        assert_eq!(i1.start_date(), NaiveDate::from_ymd_opt(2022, 1, 1));
    }

    #[test]
    fn test_end_date() {
        let i1 = Int {
            start: Bound::Included(NaiveDate::from_ymd(2022, 1, 1)),
            end: Bound::Included(NaiveDate::from_ymd(2022, 12, 31)),
        };

        assert_eq!(i1.end_date(), NaiveDate::from_ymd_opt(2022, 12, 31));
    }
}
