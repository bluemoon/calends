use chrono::NaiveDate;

use crate::IntervalLike;

pub trait Start: IntervalLike {
    fn start(&self) -> NaiveDate {
        self.start_opt().unwrap()
    }
}

pub trait End: IntervalLike {
    fn end(&self) -> NaiveDate {
        self.end_opt().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        interval::{marker, ClosedInterval, OpenStartInterval},
        RelativeDuration,
    };

    use super::*;

    #[test]
    fn test_all_intervals() {
        let i1 = OpenStartInterval::new(NaiveDate::from_ymd_opt(2022, 1, 1).unwrap());
        let i2 = ClosedInterval::from_start(
            NaiveDate::from_ymd_opt(2022, 1, 1).unwrap(),
            RelativeDuration::days(2),
        );

        fn interval<I: IntervalLike + marker::End>(interval: I) -> (Option<NaiveDate>, NaiveDate) {
            (interval.start_opt(), interval.end())
        }

        let i1 = interval(i1);
        assert_eq!(i1.0, None);
        assert_eq!(i1.1, NaiveDate::from_ymd_opt(2022, 1, 1).unwrap());

        let i2 = interval(i2);
        assert_eq!(i2.0, Some(NaiveDate::from_ymd_opt(2022, 1, 1).unwrap()));
        assert_eq!(i2.1, NaiveDate::from_ymd_opt(2022, 1, 3).unwrap());
    }
}
