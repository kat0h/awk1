/*
 * file: expr.rs
 * author: kota kato 2022
 * description:
 *   expression
 */

use crate::ast::def::{AWKExpr, AWKNumber, AWKString};
use crate::ast::{number::parse_number, string::parse_string};
use nom::{branch::alt, combinator::map, IResult};

pub fn parse_expr(input: &str) -> IResult<&str, AWKExpr> {
    alt((
        map(parse_number, |n: AWKNumber| -> AWKExpr {
            AWKExpr::AWKNumber(n)
        }),
        map(parse_string, |s: AWKString| -> AWKExpr {
            AWKExpr::AWKString(s)
        }),
    ))(input)
}

#[test]
fn test_parse_expr() {
    assert_eq!(
        Ok(("", AWKExpr::AWKNumber(parse_number("123").unwrap().1))),
        parse_expr("123")
    );
    assert_eq!(
        Ok(("", AWKExpr::AWKString(parse_string("\"hoge\"").unwrap().1))),
        parse_expr("\"hoge\"")
    );
}
