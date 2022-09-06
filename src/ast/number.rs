use std::{
    num::{ParseFloatError, ParseIntError},
    ops::Not,
};

use nom::{
    branch::{alt, permutation},
    character::complete::{char, digit1, one_of},
    combinator::map_res,
    combinator::{not, opt},
    IResult,
};

fn parse_e(input: &str) -> IResult<&str, i64> {
    map_res(
        permutation((one_of("eE"), opt(permutation((opt(one_of("+-")), digit1))))),
        |(_, e): (char, Option<(Option<char>, &str)>)| -> Result<i64, ParseIntError> {
            let (sign, int) = e.unwrap_or_else(|| (None, "0"));
            let sign: i64 = if sign.unwrap_or('+') == '+' { 1 } else { -1 };
            let int: i64 = int.parse::<i64>()?;
            Ok(sign * int)
        },
    )(input)
}

#[test]
fn test_parse_e() {
    assert_eq!(Ok(("", 1)), parse_e("e1"));
    assert_eq!(Ok(("", 1)), parse_e("E1"));
    assert_eq!(Ok(("", 0)), parse_e("e"));
    assert_eq!(Ok(("", 1)), parse_e("e+1"));
    assert_eq!(Ok(("", -1)), parse_e("e-1"));
}

fn parse_float(input: &str) -> IResult<&str, f64> {
    // 1.1 -1.1 +1.1
    // .14 +.14 -.14
    // 1. -1. +1.
    map_res(
        permutation((
            opt(one_of("+-")),
            opt(digit1),
            char('.'),
            opt(digit1),
        )),
        |(sign, int, _, frac): (
            Option<char>,
            Option<&str>,
            char,
            Option<&str>,
        )|
         -> Result<f64, ParseFloatError> {
            if int == None && frac == None {
                "".parse::<f64>()?;
            }
            let sign: f64 = match sign.unwrap_or('+') {
                '+' => 1.0,
                _ => -1.0,
            };
            let integer: f64 = int.unwrap_or("0").parse::<f64>()?;
            let fraction: f64 = frac.unwrap_or("0").parse::<f64>()? / 10_f64.powf(frac.unwrap_or("0").len() as f64);

            Ok((integer + fraction).copysign(sign))
        },
    )(input)
}

#[test]
fn test_parse_float() {
    assert_eq!(Ok(("", 1.1)), parse_float("1.1"));
    assert_eq!(Ok(("", 1.2)), parse_float("+1.2"));
    assert_eq!(Ok(("", -1.3)), parse_float("-1.3"));

    assert_eq!(Ok(("", 0.1)), parse_float(".1"));
    assert_eq!(Ok(("", 0.2)), parse_float("+.2"));
    assert_eq!(Ok(("", -0.3)), parse_float("-.3"));

    assert_eq!(Ok(("", 1.0)), parse_float("1."));
    assert_eq!(Ok(("", 1.0)), parse_float("+1."));
    assert_eq!(Ok(("", -1.0)), parse_float("-1."));

    assert!(parse_float("1").is_err());
    assert!(parse_float("-1").is_err());
    assert!(parse_float(".").is_err());
}

fn parse_int(input: &str) -> IResult<&str, i64> {
    map_res(
        permutation((opt(one_of("+-")), digit1, not(char('.')))),
        |(sign, int, _): (Option<char>, &str, ())| -> Result<i64, ParseIntError> {
            let sign: i64 = match sign.unwrap_or('+') {
                '+' => 1,
                _ => -1,
            };
            let integer: i64 = int.parse::<i64>()?;

            Ok(sign * integer)
        },
    )(input)
}

#[test]
fn test_parse_int() {
    assert_eq!(Ok(("", 1)), parse_int("1"));
    assert_eq!(Ok(("", -1)), parse_int("-1"));

    assert!(parse_int("1.0").is_err());
}

#[derive(Debug)]
pub struct AwkNumber {
    int: i64,
    float: f64,
    is_float: bool,
}

impl AwkNumber {
    fn int(value: i64) -> AwkNumber {
        AwkNumber {
            int: value,
            float: 0.0,
            is_float: false,
        }
    }
    fn float(value: f64) -> AwkNumber {
        if value == value as i64 as f64 {
            return AwkNumber {
                int: value as i64,
                float: 0.0,
                is_float: false,
            };
        } else {
            return AwkNumber {
                int: 0,
                float: value,
                is_float: true,
            };
        }
    }
}

#[test]
fn test_awk_number() {
    assert!({
        let n = AwkNumber::int(1);
        n.is_float == false && n.int == 1
    });
    assert!({
        let n = AwkNumber::float(1.0);
        n.is_float == false && n.int == 1
    });
    assert!({
        let n = AwkNumber::float(1.4);
        n.is_float == true && n.float == 1.4
    });
    assert!({
        let n = AwkNumber::float(1e40);
        n.is_float == true && n.float == 1e40
    });
}

fn parse(input: &str) -> IResult<&str, AwkNumber> {
    alt((
        map_res(
            permutation((
                parse_float,
                opt(parse_e)
            )),
            |(val, e): (f64, Option<i64>)| -> Result<AwkNumber, ()> {
                let e: f64 = 10f64.powf(e.unwrap_or(0) as f64);
                Ok(AwkNumber::float(val * e))
            }
        ),
        map_res(
            permutation((
                parse_int,
                opt(parse_e)
            )),
            |(val, e): (i64, Option<i64>)| -> Result<AwkNumber, ()> {
                let e: f64 = 10f64.powf(e.unwrap_or(0) as f64);
                return if e == 1.0 {
                    Ok(AwkNumber::int(val))
                } else {
                    Ok(AwkNumber::float(val as f64 * e))
                };
            }
        )
    ))(input)
}

#[test]
fn test_parse() {
    assert!({
        let n = parse("-1.").unwrap().1;
        !n.is_float && n.int == -1
    });
    assert!({
        let n = parse(".1").unwrap().1;
        n.is_float && n.float == 0.1
    });
    assert!({
        let n = parse("-1.0").unwrap().1;
        !n.is_float && n.int == -1
    });
    assert!({
        let n = parse("-1.2").unwrap().1;
        n.is_float && n.float == -1.2
    });
    assert!({
        let n = parse("-1.2e1").unwrap().1;
        !n.is_float && n.int == -12
    });
    assert!({
        let n = parse("1.0e-1").unwrap().1;
        n.is_float && n.float == 1.0e-1
    });
    assert!({
        let n = parse("1.0e-10").unwrap().1;
        n.is_float && n.float == 1.0e-10
    });
    assert!({
        let n = parse("0.1e2").unwrap().1;
        !n.is_float && n.int == 10
    });
}

