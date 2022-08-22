use chrono::{Datelike, NaiveDate};

use super::domain::CalendarUnit;

pub fn convert_to_quarter(date: NaiveDate) -> CalendarUnit {
    CalendarUnit::Quarter(
        date.year(),
        ((date.month() - 1) / 3 + 1).try_into().unwrap(),
    )
}

pub fn convert_to_half(date: NaiveDate) -> CalendarUnit {
    CalendarUnit::Half(
        date.year(),
        ((date.month() - 1) / 6 + 1).try_into().unwrap(),
    )
}

pub fn convert_to_month(date: NaiveDate) -> CalendarUnit {
    CalendarUnit::Month(date.year(), date.month().try_into().unwrap())
}

pub fn convert_to_iso_week(date: NaiveDate) -> CalendarUnit {
    CalendarUnit::Week(date.year(), date.iso_week().week().try_into().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group_quarter() {
        assert_eq!(
            convert_to_quarter(NaiveDate::from_ymd(2020, 2, 29)),
            CalendarUnit::Quarter(2020, 1)
        );

        assert_eq!(
            convert_to_quarter(NaiveDate::from_ymd(2022, 12, 31)),
            CalendarUnit::Quarter(2022, 4)
        )
    }

    #[test]
    fn test_group_half() {
        assert_eq!(
            convert_to_half(NaiveDate::from_ymd(2020, 2, 29)),
            CalendarUnit::Half(2020, 1)
        );

        assert_eq!(
            convert_to_half(NaiveDate::from_ymd(2022, 12, 31)),
            CalendarUnit::Half(2022, 2)
        )
    }
}
