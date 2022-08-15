use chrono::NaiveDate;

use crate::IntervalLike;

pub trait Start: IntervalLike {
    fn start(&self) -> NaiveDate {
        self.start_date().unwrap()
    }
}

pub trait End: IntervalLike {
    fn end(&self) -> NaiveDate {
        self.end_date().unwrap()
    }
}
