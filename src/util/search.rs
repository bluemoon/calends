use chrono::{Datelike, Duration, NaiveDate, Weekday};

use crate::shift;

// Borrowed from bdays
pub fn days_in_month(year: i32, month: u32) -> u32 {
    NaiveDate::from_ymd(
        match month {
            12 => year + 1,
            _ => year,
        },
        match month {
            12 => 1,
            _ => month + 1,
        },
        1,
    )
    .signed_duration_since(NaiveDate::from_ymd(year, month, 1))
    .num_days() as u32
}

pub fn find_weekday_ascending(weekday: Weekday, yy: i32, mm: u32, occurrence: u32) -> NaiveDate {
    let anchor = NaiveDate::from_ymd(yy, mm, 1);
    let mut offset = (weekday.number_from_monday() + 7 - anchor.weekday().number_from_monday()) % 7;

    if occurrence > 1 {
        offset += 7 * (occurrence - 1);
    }

    anchor + Duration::days(offset as i64)
}

pub fn find_weekday_descending(weekday: Weekday, yy: i32, mm: u32, occurrence: u32) -> NaiveDate {
    let anchor = month_end(yy, mm);
    let mut offset = (anchor.weekday().number_from_monday() + 7 - weekday.number_from_monday()) % 7;

    if occurrence > 1 {
        offset += 7 * (occurrence - 1);
    }

    anchor - Duration::days(offset as i64)
}
// End Borrowed

/// Weeks in year
pub fn weeks_in_year(date: &NaiveDate) -> u32 {
    NaiveDate::from_ymd(date.year(), 12, 31).iso_week().week()
}

/// Returns the quarter start month
#[inline]
pub fn quarter_month(date: &NaiveDate) -> u32 {
    1 + 3 * ((date.month() - 1) / 3)
}

#[inline]
pub fn month_end(mut yy: i32, mut mm: u32) -> NaiveDate {
    if mm == 12 {
        yy += 1;
        mm = 1;
    } else {
        mm += 1;
    }

    NaiveDate::from_ymd(yy, mm, 1).pred()
}

#[inline]
pub fn beginning_of_quarter(d: &NaiveDate) -> NaiveDate {
    NaiveDate::from_ymd(d.year(), quarter_month(d), 1)
}

#[inline]
pub fn beginning_of_year(d: &NaiveDate) -> NaiveDate {
    NaiveDate::from_ymd(d.year(), 1, 1)
}

#[inline]
pub fn beginning_of_month(d: &NaiveDate) -> NaiveDate {
    NaiveDate::from_ymd(d.year(), d.month(), 1)
}

/// Beginning of a biweek
///
/// Biweek 1: week 1 - week 2
/// Biweek 26: week 51 - week 52
///
/// N.B. This makes the assumption that weekdays start on Monday
///
#[inline]
pub fn beginning_of_biweek(d: &NaiveDate) -> NaiveDate {
    let beginning = if d.iso_week().week() % 2 == 0 {
        NaiveDate::from_isoywd(d.iso_week().year(), d.iso_week().week(), Weekday::Mon)
            - Duration::weeks(1)
    } else {
        NaiveDate::from_isoywd(d.iso_week().year(), d.iso_week().week(), Weekday::Mon)
    };

    debug_assert!(
        d >= &beginning,
        "date: {} was before the beginning of the biweek: {}",
        d,
        beginning
    );

    beginning
}

/// Beginning of a week
///
/// N.B. This makes the assumption that weekdays start on Monday
///
#[inline]
pub fn beginning_of_week(d: &NaiveDate) -> NaiveDate {
    NaiveDate::from_isoywd(d.iso_week().year(), d.iso_week().week(), Weekday::Mon)
}

#[inline]
pub fn end_of_year(d: &NaiveDate) -> NaiveDate {
    NaiveDate::from_ymd(d.year(), 12, 31)
}

#[inline]
pub fn end_of_quarter(d: &NaiveDate) -> NaiveDate {
    shift::shift_quarters(*d, 1).pred()
}

#[inline]
pub fn end_of_month(d: &NaiveDate) -> NaiveDate {
    let month = d.month();
    let year = d.year();
    let days_in_month = days_in_month(year, month);

    NaiveDate::from_ymd(year, month, days_in_month)
}

#[inline]
pub fn end_of_biweek(d: &NaiveDate) -> NaiveDate {
    shift::add_biweek_duration(beginning_of_biweek(d)).pred()
}

#[inline]
pub fn end_of_week(d: &NaiveDate) -> NaiveDate {
    NaiveDate::from_isoywd(d.iso_week().year(), d.iso_week().week(), Weekday::Sun)
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{Arbitrary, Gen};
    use quickcheck_macros::quickcheck;

    #[derive(Clone, Debug)]
    struct NaiveDateWrapper(NaiveDate);

    #[test]
    fn test_beginning_of_biweek() {
        assert_eq!(
            beginning_of_biweek(&NaiveDate::from_ymd(2022, 1, 1)),
            NaiveDate::from_ymd(2021, 12, 20)
        )
    }

    #[quickcheck]
    fn test_add_month_quickcheck(d: NaiveDateWrapper) {
        shift::shift_months(d.0, 1);
    }

    #[ignore = "broken af"]
    #[quickcheck]
    fn test_add_quarter_quickcheck(d: NaiveDateWrapper) {
        shift::shift_quarters(d.0, 1);
    }

    #[quickcheck]
    fn test_beginning_of_biweek_quickcheck(d: NaiveDateWrapper) {
        beginning_of_biweek(&d.0);
    }

    impl Arbitrary for NaiveDateWrapper {
        fn arbitrary(g: &mut Gen) -> NaiveDateWrapper {
            #[allow(clippy::min_max)]
            let year = std::cmp::max(std::cmp::min(i32::arbitrary(g), 1584), 2800);
            let month = 1 + u32::arbitrary(g) % 12;
            let day = 1 + u32::arbitrary(g) % 31;

            let first_date = NaiveDate::from_ymd_opt(year, month, day);
            if day > 27 {
                let result = vec![
                    first_date,
                    NaiveDate::from_ymd_opt(year, month, day - 1),
                    NaiveDate::from_ymd_opt(year, month, day - 2),
                ]
                .into_iter()
                .flatten()
                .next()
                .unwrap();

                NaiveDateWrapper(result)
            } else {
                NaiveDateWrapper(first_date.unwrap())
            }
        }
    }
}
