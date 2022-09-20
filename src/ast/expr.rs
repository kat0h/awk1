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
    branch::alt, character::complete::char, combinator::map, multi::many0, sequence::{tuple, delimited}, IResult,
};

use super::def::AWKOperation;

pub fn parse_expr(input: &str) -> IResult<&str, Box<AWKExpr>> {
    expr1(input)
}

// + -
fn expr1(input: &str) -> IResult<&str, Box<AWKExpr>> {
    map(
        tuple((
            expr2,
            many0(tuple((alt((char('+'), char('-'))), expr2))),
        )),
        |(expr, exprs): (Box<AWKExpr>, Vec<(char, Box<AWKExpr>)>)| -> Box<AWKExpr> {
            // [1, 2, 3, 4] -> [[[1, 2], 3], 4]
            let mut i = expr;
            for j in exprs {
                match j {
                    ('+', k) => {
                        i = Box::new(AWKExpr::BinaryOperation {
                            op: AWKOperation::Add,
                            left: i,
                            right: k
                        });
                    },
                    ('-', k) => {
                        i = Box::new(AWKExpr::BinaryOperation {
                            op: AWKOperation::Sub,
                            left: i,
                            right: k
                        })
                    },
                    _ => panic!(),
                }
            };
            return i;
        },
    )(input)
}

// * /
fn expr2(input: &str) -> IResult<&str, Box<AWKExpr>> {
    map(
        tuple((
            expr3,
            many0(tuple((alt((char('*'), char('/'))), expr3))),
        )),
        |(expr, exprs): (Box<AWKExpr>, Vec<(char, Box<AWKExpr>)>)| -> Box<AWKExpr> {
            let mut i = expr;
            for j in exprs {
                match j {
                    ('*', k) => {
                        i = Box::new(AWKExpr::BinaryOperation {
                            op: AWKOperation::Mul,
                            left: i,
                            right: k
                        });
                    },
                    ('/', k) => {
                        i = Box::new(AWKExpr::BinaryOperation {
                            op: AWKOperation::Div,
                            left: i,
                            right: k
                        })
                    },
                    _ => panic!(),
                }
            };
            return i;
        },
    )(input)
}

fn expr3(input: &str) -> IResult<&str, Box<AWKExpr>> {
    alt((
        value,
        delimited(char('('), expr1, char(')'))
    ))(input)
}

fn value(input: &str) -> IResult<&str, Box<AWKExpr>> {
    map(
        parse_value,
        |val: AWKValue| -> Box<AWKExpr> {
            Box::new(AWKExpr::Value(val))
        }
    )(input)
}

#[test]
fn test_parse_expr() {
    dbg!(parse_expr("123-444*(555-666)--2133").unwrap());
}
