/*
 * file: expr.rs
 * author: kota kato 2022
 * description:
 *   expression
 */

use crate::ast::{
    def::{AWKExpr, AWKValue},
    value::parse_value,
};
use nom::{combinator::map, IResult};

pub fn parse_expr(input: &str) -> IResult<&str, AWKExpr> {
    map(parse_value, |n: AWKValue| -> AWKExpr {
        AWKExpr::AWKValue(n)
    })(input)
}

#[test]
fn test_parse_expr() {
    assert_eq!(
        Ok(("", AWKExpr::AWKValue(parse_value("123").unwrap().1))),
        parse_expr("123")
    );
    assert_eq!(
        Ok(("", AWKExpr::AWKValue(parse_value("\"hoge\"").unwrap().1))),
        parse_expr("\"hoge\"")
    );
}
