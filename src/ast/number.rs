//  123456
// -123456
// +123456
//
// 3.14
// .14
// 3.14e1

use std::num::{ParseFloatError, ParseIntError};

use nom::{
    branch::permutation,
    character::complete::{char, digit1, one_of},
    combinator::map_res,
    combinator::opt,
    IResult,
};

/*
#[derive(Debug)]
pub struct AwkNumber {
    integer: i64,
    float: f32,
    is_float: bool,
}

impl AwkNumber {
    fn new_integer(value: i64) -> AwkNumber {
        AwkNumber {
            integer: value,
            float: 0.0,
            is_float: false,
        }
    }
    fn new_float(value: f32) -> AwkNumber {
        AwkNumber {
            integer: 0,
            float: value,
            is_float: true,
        }
    }
}

#[test]
fn test_awk_number() {
    assert_eq!(1.1, AwkNumber::new_float(1.1).float)
}
*/

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
    map_res(
        permutation((
                opt(one_of("+-")),
                opt(digit1),
                char('.'),
                digit1,
                //opt(parse_e)
        )),
        |(sign, int, _, frac/*, e*/): (Option<char>, Option<&str>, char, &str/*, Option<i64>*/)| -> Result<f64, ParseFloatError> {
            let sign: f64 = match sign.unwrap_or('+') {
               '+' => 1.0,
               _ => -1.0,
            };
            let integer: f64 = int.unwrap_or("0").parse::<f64>()?;
            let fraction: f64 = frac.parse::<f64>()? / 10f64.powf(frac.len() as f64);
            //let e: f64 = 10f64.powf(e.unwrap_or(0i64) as f64);

            Ok((integer + fraction).copysign(sign))// * e)
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

    // assert_eq!(Ok(("", 1.0e-1)), parse_float("1.0e-1"));
    // assert_eq!(Ok(("", 1.0e-10)), parse_float("1.0e-10"));
    // assert_eq!(Ok(("", 100.0)), parse_float("1.0e2"));
}


// fn parse_int(input: &str) -> IResult<&str, i64> {
//     map_res(
//         permutation((
//         )),
//         || {
//         }
//     )(input)
// }
