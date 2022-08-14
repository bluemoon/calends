use chrono::{Datelike, NaiveDate};

use crate::util;

/// Shift a month duration to the current date
///
/// This function adds one month, it does not add 30 days or 31 days
///
/// # Examples
///
/// ```
/// # use chrono::NaiveDate;
/// # use calends::util::shift_months;
///
/// let n1 = shift_months(NaiveDate::from_ymd(2022, 1, 1), 1);
/// assert_eq!(n1, NaiveDate::from_ymd(2022, 2, 1));
///
/// let n2 = shift_months(NaiveDate::from_ymd(2022, 2, 3), 2);
/// assert_eq!(n2, NaiveDate::from_ymd(2022, 4, 3));
///
/// let n3 = shift_months(NaiveDate::from_ymd(2022, 2, 3), -1);
/// assert_eq!(n3, NaiveDate::from_ymd(2022, 1, 3));
/// ```
///
/// The behavior for end of month works as follows:
///
/// ```
/// # use chrono::NaiveDate;
/// # use calends::util::shift_months;
///
/// assert_eq!(
///   shift_months(NaiveDate::from_ymd(2022, 2, 28), 1),
///   NaiveDate::from_ymd(2022, 3, 31)
/// );
/// assert_eq!(
///   shift_months(NaiveDate::from_ymd(2022, 3, 31), 1),
///   NaiveDate::from_ymd(2022, 4, 30)
/// );
/// ```
///
/// This also works across year boundaries:
///
/// ```
/// # use chrono::NaiveDate;
/// # use calends::util::shift_months;
///
/// let n4 = shift_months(NaiveDate::from_ymd(2022, 2, 28), 11);
/// assert_eq!(n4, NaiveDate::from_ymd(2023, 1, 31));
/// ```
///
///
#[inline]
pub fn shift_months(date: NaiveDate, months: i32) -> NaiveDate {
    let mut year = date.year() + (date.month() as i32 + months) / 12;
    let mut month = (date.month() as i32 + months) % 12;

    if month < 1 {
        year -= 1;
        month += 12;
    }

    let date_end_of_month = util::month_end(date.year(), date.month());
    let day = if date_end_of_month.day() == date.day() {
        // if the current date is the last date of the month, the next month will need to be the
        // last date as well
        util::month_end(year, month as u32).day()
    } else {
        // get the maximum of the month and clamp it to that, we cannot exceed the end of the current
        // month
        std::cmp::min(date.day(), util::month_end(year, month as u32).day())
    };
    NaiveDate::from_ymd(year, month as u32, day)
}

/// Add a quarter to the date supplied
///
/// A quarter refers to one-fourth of a year and is typically expressed as Q1 for the first
/// quarter, etc., and can be expressed with the year, such as Q1 2021 (or Q121).
///
/// If the current date falls in the last quarter of the year, this will shift to the first quarter
/// of the next year.
///
/// # Examples
///
/// ```ignore
/// # use chrono::NaiveDate;
/// # use dateutil::addition;
///
/// assert_eq!(addition::add_quarter_duration(NaiveDate::from_ymd(2022, 1, 1)), NaiveDate::from_ymd(2022, 4, 1));
/// assert_eq!(addition::add_quarter_duration(NaiveDate::from_ymd(2022, 11, 3)), NaiveDate::from_ymd(2023, 2, 3));
///
/// ```
#[inline]
pub fn shift_quarters(date: NaiveDate, quarters: i32) -> NaiveDate {
    shift_months(date, 3 * quarters)
}

/// Adds a year to the current date
///
/// # Examples
///
/// ```
/// # use chrono::NaiveDate;
/// # use calends::shift_years;
///
/// let n1 = shift_years(NaiveDate::from_ymd(2022, 1, 1), 1);
/// let n2 = shift_years(NaiveDate::from_ymd(1584, 2, 3), -1);
///
/// assert_eq!(n1, NaiveDate::from_ymd(2023, 1, 1));
/// assert_eq!(n2, NaiveDate::from_ymd(1583, 2, 3));
///
/// ```
#[inline]
pub fn shift_years(date: NaiveDate, years: i32) -> NaiveDate {
    shift_months(date, years * 12)
}

/// Add a week
///
/// Simple enough
#[inline]
pub fn shift_weeks(date: NaiveDate, delta: i32) -> NaiveDate {
    date + chrono::Duration::weeks(delta as i64)
}

/// Add a day
#[inline]
pub fn shift_days(date: NaiveDate, days: i32) -> NaiveDate {
    date + chrono::Duration::days(days.into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shift_months() {
        assert_eq!(
            shift_months(NaiveDate::from_ymd(2022, 1, 1), 1),
            NaiveDate::from_ymd(2022, 2, 1)
        );

        assert_eq!(
            shift_months(NaiveDate::from_ymd(2022, 1, 1), -1),
            NaiveDate::from_ymd(2021, 12, 1)
        )
    }

    #[test]
    fn test_shift_quarters() {
        assert_eq!(
            shift_quarters(NaiveDate::from_ymd(2022, 1, 1), 1),
            NaiveDate::from_ymd(2022, 4, 1)
        );
    }

    #[test]
    fn test_shift_years() {
        assert_eq!(
            shift_years(NaiveDate::from_ymd(2022, 1, 1), 1),
            NaiveDate::from_ymd(2023, 1, 1)
        );

        assert_eq!(
            shift_years(NaiveDate::from_ymd(2024, 2, 29), 1),
            NaiveDate::from_ymd(2025, 2, 28)
        );
    }
}
