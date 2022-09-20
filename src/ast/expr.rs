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
use nom::{
    branch::alt, character::complete::char, combinator::map, multi::many0, sequence::tuple, IResult,
};

use super::def::AWKOperation;

pub fn parse_expr(input: &str) -> IResult<&str, Box<AWKExpr>> {
    expr1(input)
}

// + -
fn expr1(input: &str) -> IResult<&str, Box<AWKExpr>> {
    map(
        tuple((
            value,
            many0(tuple((alt((char('+'), char('-'))), value))),
        )),
        |(expr, exprs): (AWKExpr, Vec<(char, AWKExpr)>)| -> Box<AWKExpr> {
            // [1, 2, 3, 4] -> [[[1, 2], 3], 4]
            let mut i = Box::new(expr);
            for j in exprs {
                match j {
                    ('+', k) => {
                        i = Box::new(AWKExpr::BinaryOperation {
                            op: AWKOperation::Plus,
                            left: i,
                            right: Box::new(k)
                        });
                    },
                    ('-', k) => {
                        i = Box::new(AWKExpr::BinaryOperation {
                            op: AWKOperation::Minus,
                            left: i,
                            right: Box::new(k)
                        })
                    },
                    _ => panic!(),
                }
            };
            return i;
        },
    )(input)
}

fn value(input: &str) -> IResult<&str, AWKExpr> {
    map(
        parse_value,
        |val: AWKValue| -> AWKExpr {
            AWKExpr::Value(val)
        }
    )(input)
}

#[test]
fn test_parse_expr() {
    dbg!(parse_expr("123-444+555-666")).unwrap();
}
