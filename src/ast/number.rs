/*
 * file: number.rs
 * author: kota kato 2022
 * description:
 *   Parse number literal
 */

use crate::ast::def::AWKNumber;
use std::num::{ParseFloatError, ParseIntError};

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

pub fn parse_number(input: &str) -> IResult<&str, AWKNumber> {
    alt((
        map_res(
            // parse float sintax
            permutation((parse_float, opt(parse_e))),
            |(val, e): (f64, Option<i64>)| -> Result<AWKNumber, ()> {
                let e: f64 = 10f64.powf(e.unwrap_or(0) as f64);
                let r = val * e;

                // if r is integer
                return if r == r as i64 as f64 {
                    Ok(AWKNumber::Int(r as i64))
                } else {
                    Ok(AWKNumber::Float(val * e))
                };
            },
        ),
        map_res(
            // parse int sintax
            permutation((parse_int, opt(parse_e))),
            |(val, e): (i64, Option<i64>)| -> Result<AWKNumber, ()> {
                match e {
                    Some(e) => {
                        let e: f64 = 10f64.powf(e as f64);
                        let r = val as f64 * e;
                        return if r == r as i64 as f64 {
                            Ok(AWKNumber::Int(r as i64))
                        } else {
                            Ok(AWKNumber::Float(r))
                        };
                    }
                    None => Ok(AWKNumber::Int(val)),
                }
            },
        ),
    ))(input)
}

#[test]
fn test_parse_e() {
    assert_eq!(Ok(("", 1)), parse_e("e1"));
    assert_eq!(Ok(("", 1)), parse_e("E1"));
    assert_eq!(Ok(("", 0)), parse_e("e"));
    assert_eq!(Ok(("", 1)), parse_e("e+1"));
    assert_eq!(Ok(("", -1)), parse_e("e-1"));
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
    assert_eq!(Ok(("", 1)), parse_int("1"));
    assert_eq!(Ok(("", -1)), parse_int("-1"));

    assert!(parse_int("1.0").is_err());
}

#[test]
fn test_parse_number() {
    assert_eq!(Ok(("", AWKNumber::Int(-1))), parse_number("-1."),);
    assert_eq!(Ok(("", AWKNumber::Float(0.1))), parse_number(".1"),);
    assert_eq!(Ok(("", AWKNumber::Int(-1))), parse_number("-1.0"),);
    assert_eq!(Ok(("", AWKNumber::Float(-1.2))), parse_number("-1.2"),);
    assert_eq!(Ok(("", AWKNumber::Int(-12))), parse_number("-1.2e1"),);
    assert_eq!(Ok(("", AWKNumber::Float(1.0e-1))), parse_number("1.0e-1"),);
    assert_eq!(Ok(("", AWKNumber::Float(1.0e-10))), parse_number("1.0e-10"));
    assert_eq!(Ok(("", AWKNumber::Int(10))), parse_number("0.1e2"));
    assert_eq!(
        Ok(("", AWKNumber::Float(2.2250738585072013e-308))),
        parse_number("2.2250738585072013e-308")
    );
    assert_eq!(Ok((",", AWKNumber::Int(2))), parse_number("2,"),);
}
