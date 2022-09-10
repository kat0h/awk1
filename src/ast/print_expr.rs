/*
 * file: print_expr.rs
 * author: kota kato 2022
 * description:
 *  parser for expr of print statement
 */


#![allow(dead_code)]

use crate::ast::def::{AWKNonUnaryPrintExpr, AWKNumber, AWKString};
use crate::ast::{number::parse_number, string::parse_string};
use nom::{branch::alt, combinator::map, IResult};

pub fn parse_non_unary_print_expr(input: &str) -> IResult<&str, AWKNonUnaryPrintExpr> {
    alt((
        map(parse_number, |n: AWKNumber| {
            AWKNonUnaryPrintExpr::AWKNumber(n)
        }),
        map(parse_string, |s: AWKString| {
            AWKNonUnaryPrintExpr::AWKString(s)
        }),
    ))(input)
}

#[test]
fn test_awk_non_unary_print_expr() {
    let e = AWKNonUnaryPrintExpr::AWKString(AWKString {
        value: "hoge".to_string(),
    });
    let a = parse_non_unary_print_expr("\"hoge\"");
    assert_eq!(Ok(("", e)), a);

    let e = AWKNonUnaryPrintExpr::AWKNumber(AWKNumber {
        int: 100,
        float: 0.0,
        is_float: false,
    });
    let a = parse_non_unary_print_expr("1e2");
    assert_eq!(Ok(("", e)), a);
}
