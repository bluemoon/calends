use chrono::{Datelike, NaiveDate};

use super::domain::CalendarUnit;

/// Convert a date into a year
pub fn convert_to_year(date: NaiveDate) -> CalendarUnit {
    CalendarUnit::Year(date.year())
}

/// Convert a date into a quarter
pub fn convert_to_quarter(date: NaiveDate) -> CalendarUnit {
    CalendarUnit::Quarter(
        date.year(),
        ((date.month() - 1) / 3 + 1).try_into().unwrap(),
    )
}

/// Convert a date into a half
pub fn convert_to_half(date: NaiveDate) -> CalendarUnit {
    CalendarUnit::Half(
        date.year(),
        ((date.month() - 1) / 6 + 1).try_into().unwrap(),
    )
}

/// Convert a date into a month
pub fn convert_to_month(date: NaiveDate) -> CalendarUnit {
    CalendarUnit::Month(date.year(), date.month().try_into().unwrap())
}

/// Convert a date into an ISO week
pub fn convert_to_iso_week(date: NaiveDate) -> CalendarUnit {
    CalendarUnit::Week(date.year(), date.iso_week().week().try_into().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_week() {
        assert_eq!(
            convert_to_iso_week(NaiveDate::from_ymd_opt(2020, 2, 29).unwrap()),
            CalendarUnit::Week(2020, 9)
        );

        assert_eq!(
            convert_to_iso_week(NaiveDate::from_ymd_opt(2022, 12, 31).unwrap()),
            CalendarUnit::Week(2022, 52)
        )
    }

    #[test]
    fn test_convert_month() {
        assert_eq!(
            convert_to_month(NaiveDate::from_ymd_opt(2020, 2, 29).unwrap()),
            CalendarUnit::Month(2020, 2)
        );

        assert_eq!(
            convert_to_month(NaiveDate::from_ymd_opt(2022, 12, 31).unwrap()),
            CalendarUnit::Month(2022, 12)
        )
    }

    #[test]
    fn test_convert_quarter() {
        assert_eq!(
            convert_to_quarter(NaiveDate::from_ymd_opt(2020, 2, 29).unwrap()),
            CalendarUnit::Quarter(2020, 1)
        );

        assert_eq!(
            convert_to_quarter(NaiveDate::from_ymd_opt(2022, 12, 31).unwrap()),
            CalendarUnit::Quarter(2022, 4)
        )
    }

    #[test]
    fn test_convert_half() {
        assert_eq!(
            convert_to_half(NaiveDate::from_ymd_opt(2020, 2, 29).unwrap()),
            CalendarUnit::Half(2020, 1)
        );

        assert_eq!(
            convert_to_half(NaiveDate::from_ymd_opt(2022, 12, 31).unwrap()),
            CalendarUnit::Half(2022, 2)
        )
    }

    #[test]
    fn test_convert_year() {
        assert_eq!(
            convert_to_year(NaiveDate::from_ymd_opt(2020, 2, 29).unwrap()),
            CalendarUnit::Year(2020)
        );

        assert_eq!(
            convert_to_year(NaiveDate::from_ymd_opt(2022, 12, 31).unwrap()),
            CalendarUnit::Year(2022)
        )
    }
}
