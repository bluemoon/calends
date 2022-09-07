use nom::{
    bytes::complete::{take_while, take_while_m_n},
    character::{complete::char, is_digit},
    combinator::opt,
    error::Error,
    Err, IResult,
};

pub fn take_signed_digits(i: &[u8]) -> IResult<&[u8], i32> {
    let (i, negative) = opt(char('-'))(i)?;
    let (i, digits) = take_while(is_digit)(i)?;

    if digits.is_empty() {
        return Err(Err::Error(Error::new(i, nom::error::ErrorKind::Eof)));
    }

    let s = std::str::from_utf8(digits).expect("Invalid data, expected UTF-8 string");
    let res: i32 = s
        .parse()
        .expect("Invalid string, expected ASCII representation of a number");

    match negative {
        Some(_) => Ok((i, -res)),
        None => Ok((i, res)),
    }
}

pub fn take_n_digits(i: &[u8], n: usize) -> IResult<&[u8], u32> {
    let (i, digits) = take_while_m_n(n, n, is_digit)(i)?;

    let s = std::str::from_utf8(digits).expect("Invalid data, expected UTF-8 string");
    let res = s
        .parse()
        .expect("Invalid string, expected ASCII representation of a number");

    Ok((i, res))
}
