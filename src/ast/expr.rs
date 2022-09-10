#![allow(dead_code)]

use crate::ast::def::{AWKExpr, AWKNumber, AWKString};
use crate::ast::{number::parse_number, string::parse_string};
use nom::{branch::alt, combinator::map, IResult};

pub fn parse_expr(input: &str) -> IResult<&str, AWKExpr> {
    alt((
        map(parse_number, |n: AWKNumber| AWKExpr::AWKNumber(n)),
        map(parse_string, |s: AWKString| AWKExpr::AWKString(s)),
    ))(input)
}

#[test]
fn test_awk_expr() {
    let e = AWKExpr::AWKString(AWKString {
        value: "hoge".to_string(),
    });
    let a = parse_expr("\"hoge\"");
    assert_eq!(Ok(("", e)), a);

    let e = AWKExpr::AWKNumber(AWKNumber {
        int: 100,
        float: 0.0,
        is_float: false,
    });
    let a = parse_expr("1e2");
    assert_eq!(Ok(("", e)), a);
}
