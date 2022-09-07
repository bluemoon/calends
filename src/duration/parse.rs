use nom::{
    bytes::complete::tag,
    character::complete::one_of,
    combinator::opt,
    error::Error,
    multi::count,
    sequence::{preceded, tuple},
    Err, IResult,
};

use crate::{parser::take_signed_digits, RelativeDuration};

#[derive(Debug, PartialEq)]
pub enum Unit {
    Years(i32),
    Months(i32),
    Weeks(i32),
    Days(i32),
}

fn parse_duration_chunk(input: &[u8]) -> IResult<&[u8], Unit> {
    let (i, (amt, u)) = tuple((take_signed_digits, one_of("YMWD")))(input)?;
    match u {
        'Y' => Ok((i, Unit::Years(amt))),
        'M' => Ok((i, Unit::Months(amt))),
        'W' => Ok((i, Unit::Weeks(amt))),
        'D' => Ok((i, Unit::Days(amt))),
        _ => Err(Err::Error(Error::new(i, nom::error::ErrorKind::Fail))),
    }
}

/// Parse an ISO8601-2:2019 duration
///
/// Returns the leftovers for use in combination with other parsers
pub fn parse_relative_duration(input: &[u8]) -> IResult<&[u8], RelativeDuration> {
    let (leftover, units) = preceded(tag("P"), count(opt(parse_duration_chunk), 4))(input)?;

    let rd = units
        .iter()
        .flatten()
        .fold(RelativeDuration::default(), |start, unit| match unit {
            Unit::Years(y) => start.with_months(y * 12),
            Unit::Months(m) => start.with_months(*m),
            Unit::Weeks(w) => start.with_weeks(*w),
            Unit::Days(d) => start.with_days(*d),
        });

    Ok((leftover, rd))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_duration_chunk() {
        let (_input, duration) = parse_duration_chunk("2Y".as_bytes()).unwrap();
        assert_eq!(duration, Unit::Years(2))
    }

    #[test]
    fn test_parse_duration_chunk_months() {
        let (_input, duration) = parse_duration_chunk("2M".as_bytes()).unwrap();
        assert_eq!(duration, Unit::Months(2))
    }

    #[test]
    fn test_parse_duration_chunk_weeks() {
        let (_input, duration) = parse_duration_chunk("-1W".as_bytes()).unwrap();
        assert_eq!(duration, Unit::Weeks(-1))
    }

    #[test]
    fn test_parse_duration_chunk_days() {
        let (_input, duration) = parse_duration_chunk("180D".as_bytes()).unwrap();
        assert_eq!(duration, Unit::Days(180))
    }

    #[test]
    fn test_parse_duration() {
        let (_input, duration) = parse_relative_duration("P3W2D".as_bytes()).unwrap();
        assert_eq!(
            duration,
            RelativeDuration::default().with_weeks(3).with_days(2)
        )
    }
}
