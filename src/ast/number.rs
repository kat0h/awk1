/*
 * file: number.rs
 * author: kota kato 2022
 * description:
 *   Parse number literal
 */

use crate::ast::def::AWKFloat;
use std::num::{ParseFloatError, ParseIntError};

use nom::{
    branch::alt,
    character::complete::{char, digit1, one_of},
    combinator::{map, map_res,  not, opt},
    sequence::tuple,
    IResult,
};

fn parse_e(input: &str) -> IResult<&str, f64> {
    map_res(
        tuple((one_of("eE"), opt(tuple((opt(one_of("+-")), digit1))))),
        |(_, e): (char, Option<(Option<char>, &str)>)| -> Result<f64, ParseIntError> {
            let (sign, int) = e.unwrap_or_else(|| (None, "0"));
            let sign: i64 = if sign.unwrap_or('+') == '+' { 1 } else { -1 };
            let int: i64 = int.parse::<i64>()?;
            Ok((sign * int) as f64)
        },
    )(input)
}

fn parse_float(input: &str) -> IResult<&str, f64> {
    // 1.1 -1.1 +1.1
    // .14 +.14 -.14
    // 1. -1. +1.
    map_res(
        tuple((
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

fn parse_int(input: &str) -> IResult<&str, f64> {
    map_res(
        tuple((opt(one_of("+-")), digit1, not(char('.')))),
        |(sign, int, _): (Option<char>, &str, ())| -> Result<f64, ParseIntError> {
            let sign: i64 = match sign.unwrap_or('+') {
                '+' => 1,
                _ => -1,
            };
            let integer: i64 = int.parse::<i64>()?;

            Ok((sign * integer) as f64)
        },
    )(input)
}

pub fn parse_number(input: &str) -> IResult<&str, AWKFloat> {
    alt((
        map(
            // parse float sintax
            tuple((parse_float, opt(parse_e))),
            |(val, e): (f64, Option<f64>)| -> AWKFloat {
                let e = 10_f64.powf(e.unwrap_or(0.0));
                return val * e;
            }
        ),
        map(
            // parse int sintax
            tuple((parse_int, opt(parse_e))),
            |(val, e): (f64, Option<f64>)| -> AWKFloat {
                match e {
                    Some(e) => {
                        let e = 10f64.powf(e);
                        val * e
                    },
                    None => val,
                }
            }
        ),
    ))(input)
}

#[test]
fn test_parse_e() {
    assert_eq!(Ok(("", 1.0)), parse_e("e1"));
    assert_eq!(Ok(("", 1.0)), parse_e("E1"));
    assert_eq!(Ok(("", 0.0)), parse_e("e"));
    assert_eq!(Ok(("", 1.0)), parse_e("e+1"));
    assert_eq!(Ok(("", -1.0)), parse_e("e-1"));
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

#[test]
fn test_parse_int() {
    assert_eq!(Ok(("", 1.0)), parse_int("1"));
    assert_eq!(Ok(("", -1.0)), parse_int("-1"));

    assert!(parse_int("1.0").is_err());
}

#[test]
fn test_parse_number() {
    assert_eq!(Ok(("", -1.0)), parse_number("-1."));
    assert_eq!(Ok(("", 0.1)), parse_number(".1"));
    assert_eq!(Ok(("", -1.0)), parse_number("-1.0"));
    assert_eq!(Ok(("", -1.2)), parse_number("-1.2"));
    assert_eq!(Ok(("", -12.0)), parse_number("-1.2e1"));
    assert_eq!(Ok(("", 1.0e-1)), parse_number("1.0e-1"));
    assert_eq!(Ok(("", 1.0e-10)), parse_number("1.0e-10"));
    assert_eq!(Ok(("", 10.0)), parse_number("0.1e2"));
    assert_eq!(
        Ok(("", 2.2250738585072013e-308)),
        parse_number("2.2250738585072013e-308")
    );
    assert_eq!(Ok((",", 2.0)), parse_number("2,"));
}
