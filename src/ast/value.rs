/*
 * file: value.rs
 * author: kota kato 2022
 * description:
 *   value
 */

use crate::ast::def::{AWKNumber, AWKString, AWKValue};
use crate::ast::{number::parse_number, string::parse_string};
use nom::{branch::alt, combinator::map, IResult};

pub fn parse_value(input: &str) -> IResult<&str, AWKValue> {
    alt((
        map(parse_number, |n: AWKNumber| -> AWKValue {
            AWKValue::AWKNumber(n)
        }),
        map(parse_string, |s: AWKString| -> AWKValue {
            AWKValue::AWKString(s)
        }),
    ))(input)
}

#[test]
fn test_parse_value() {
    assert_eq!(
        Ok(("", AWKValue::AWKNumber(parse_number("123").unwrap().1))),
        parse_value("123")
    );
    assert_eq!(
        Ok(("", AWKValue::AWKString(parse_string("\"hoge\"").unwrap().1))),
        parse_value("\"hoge\"")
    );
}
