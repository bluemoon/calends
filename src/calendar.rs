use chrono::{Datelike, NaiveDate};

use crate::{addition, month_day::MonthDay};

pub struct Calendar {
  pub year_start: MonthDay,
}

impl Calendar {
  pub fn new(year_start: MonthDay) -> Self {
    Self { year_start }
  }

  pub fn calendar_year() -> Self {
    Calendar {
      year_start: MonthDay::new(1, 1),
    }
  }
  /// Get the current month of the current calendar for a given date
  ///
  /// # Examples
  ///
  /// ```
  /// # use chrono::NaiveDate;
  /// use dateutil::calendar::Calendar;
  /// use dateutil::month_day::MonthDay;
  ///
  /// let cd = Calendar::calendar_year().month(NaiveDate::from_ymd(2022, 3, 3));
  /// assert_eq!(cd, 3);
  ///
  /// let cd1 = Calendar::new(MonthDay::new(3, 1));
  /// // If the calendar start on the 3rd month then the first month will be on the 3rd month and
  /// // so forth
  /// assert_eq!(cd1.month(NaiveDate::from_ymd(2022, 3, 1)), 1);
  /// assert_eq!(cd1.month(NaiveDate::from_ymd(2022, 6, 1)), 4);
  /// assert_eq!(cd1.month(NaiveDate::from_ymd(2023, 1, 1)), 11);
  /// assert_eq!(cd1.month(NaiveDate::from_ymd(2023, 2, 1)), 12);
  ///
  /// ```
  #[inline]
  pub fn month(&self, date: NaiveDate) -> u32 {
    addition::add_months_duration(date, 13 - self.year_start.month()).month()
  }

  /// Get the current year of the current calendar for a given date
  ///
  /// # Examples
  ///
  /// ```ignore
  /// # use chrono::NaiveDate;
  /// use dateutil::calendar::Calendar;
  /// use dateutil::month_day::MonthDay;
  ///
  /// let cd = Calendar::calendar_year().month(NaiveDate::from_ymd(2022, 3, 3));
  /// assert_eq!(cd, 3);
  ///
  /// let cd1 = Calendar::new(MonthDay::new(11, 1));
  /// // FY22
  /// assert_eq!(cd1.year(NaiveDate::from_ymd(2021, 11, 1)), 2022);
  /// assert_eq!(cd1.year(NaiveDate::from_ymd(2022, 10, 1)), 2022);
  /// // FY23
  /// assert_eq!(cd1.year(NaiveDate::from_ymd(2022, 11, 1)), 2022);
  /// assert_eq!(cd1.year(NaiveDate::from_ymd(2022, 3, 1)), 2022);
  /// assert_eq!(cd1.year(NaiveDate::from_ymd(2022, 6, 1)), 2022);
  ///
  /// ```
  #[inline]
  pub fn year(&self, date: NaiveDate) -> i32 {
    addition::add_months_duration(date, 13 - self.year_start.month()).year()
  }

  /// Beginning of the quarter for the given calendar
  pub fn beginning_of_quarter(&self, date: NaiveDate) -> NaiveDate {
    let _quarter_start = 1 + 3 * ((self.month(date) - 1) / 3);
    NaiveDate::from_ymd(date.year(), date.month(), date.day())
  }
}
