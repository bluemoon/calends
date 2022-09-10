use chrono::NaiveDate;
use nom::{
    branch::alt,
    bytes::complete::tag,
    sequence::{preceded, terminated},
    IResult,
};

use crate::{duration::parse::parse_relative_duration, parser::take_n_digits};

use super::{BoundInterval, UnboundedEndInterval, UnboundedStartInterval};

pub fn parse_date(i: &[u8]) -> IResult<&[u8], NaiveDate> {
    let (i, year) = take_n_digits(i, 4)?;
    let (i, _) = tag(b"-")(i)?;
    let (i, month) = take_n_digits(i, 2)?;
    let (i, _) = tag(b"-")(i)?;
    let (i, day) = take_n_digits(i, 2)?;

    Ok((i, NaiveDate::from_ymd(year.try_into().unwrap(), month, day)))
}

fn parse_start_and_duration(i: &[u8]) -> IResult<&[u8], BoundInterval> {
    let (i, date) = parse_date(i)?;
    let (i, _) = tag(b"/")(i)?;
    let (i, duration) = parse_relative_duration(i)?;

    Ok((i, BoundInterval::from_start(date, duration)))
}

fn parse_start_and_end(i: &[u8]) -> IResult<&[u8], BoundInterval> {
    let (i, start) = parse_date(i)?;
    let (i, _) = tag(b"/")(i)?;
    let (i, end) = parse_date(i)?;

    Ok((i, BoundInterval::with_dates(start, end)))
}

pub fn parse_interval(i: &[u8]) -> IResult<&[u8], BoundInterval> {
    alt((parse_start_and_end, parse_start_and_duration))(i)
}

pub fn parse_open_start_interval(i: &[u8]) -> IResult<&[u8], UnboundedStartInterval> {
    let (i, date) = preceded(tag("../"), parse_date)(i)?;
    Ok((i, UnboundedStartInterval::new(date)))
}

pub fn parse_open_end_interval(i: &[u8]) -> IResult<&[u8], UnboundedEndInterval> {
    let (i, date) = terminated(parse_date, tag("../"))(i)?;
    Ok((i, UnboundedEndInterval::new(date)))
}
