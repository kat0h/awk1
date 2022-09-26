/*
 * file: expr.rs
 * author: kota kato 2022
 * description:
 *   expression
 */

use crate::ast::{
    def::{AWKExpr, AWKLval, AWKOperation, AWKVal},
    name::{parse_variable_name_expr, parse_variable_name_string},
    util::*,
    value::parse_value,
};
use nom::{
    branch::alt,
    character::complete::char,
    combinator::map,
    multi::many0,
    sequence::{delimited, tuple},
    IResult,
};

pub fn parse_expr(input: &str) -> IResult<&str, Box<AWKExpr>> {
    expr1(input)
}

/// parse assignment expression
fn expr1(input: &str) -> IResult<&str, Box<AWKExpr>> {
    // 左辺値
    let val = map(parse_variable_name_string, |name| AWKLval::Name(name));
    let field = map(tuple((char('$'), wss, expr5)), |(_, _, expr)| {
        AWKLval::Field(expr)
    });
    let lval = alt((val, field));

    let assign = map(
        tuple((lval, delimited(wss, char('='), wss), expr1)),
        |(l, _, e)| Box::new(AWKExpr::Assign { lval: l, expr: e }),
    );
    alt((assign, expr2))(input)
}

/// parse + -
fn expr2(input: &str) -> IResult<&str, Box<AWKExpr>> {
    let symbol = delimited(wss, alt((char('+'), char('-'))), wss);

    map(
        tuple((expr3, many0(tuple((symbol, expr3))))),
        |(expr, exprs): (Box<AWKExpr>, Vec<(char, Box<AWKExpr>)>)| -> Box<AWKExpr> {
            // [1, 2, 3, 4] -> [[[1, 2], 3], 4]
            let mut i = expr;
            for j in exprs {
                match j {
                    ('+', k) => {
                        i = Box::new(AWKExpr::BinaryOperation {
                            op: AWKOperation::Add,
                            left: i,
                            right: k,
                        });
                    }
                    ('-', k) => {
                        i = Box::new(AWKExpr::BinaryOperation {
                            op: AWKOperation::Sub,
                            left: i,
                            right: k,
                        })
                    }
                    _ => panic!(),
                }
            }
            return i;
        },
    )(input)
}

/// parse * /
fn expr3(input: &str) -> IResult<&str, Box<AWKExpr>> {
    let symbol = delimited(wss, alt((char('*'), char('/'))), wss);

    map(
        tuple((expr4, many0(tuple((symbol, expr4))))),
        |(expr, exprs): (Box<AWKExpr>, Vec<(char, Box<AWKExpr>)>)| -> Box<AWKExpr> {
            let mut i = expr;
            for j in exprs {
                match j {
                    ('*', k) => {
                        i = Box::new(AWKExpr::BinaryOperation {
                            op: AWKOperation::Mul,
                            left: i,
                            right: k,
                        });
                    }
                    ('/', k) => {
                        i = Box::new(AWKExpr::BinaryOperation {
                            op: AWKOperation::Div,
                            left: i,
                            right: k,
                        })
                    }
                    _ => panic!(),
                }
            }
            return i;
        },
    )(input)
}

/// parse field reference $expr
fn expr4(input: &str) -> IResult<&str, Box<AWKExpr>> {
    let field_reference = tuple((char('$'), wss, expr5));
    alt((
        expr5,
        map(
            field_reference,
            |(_, _, record): (char, _, Box<AWKExpr>)| -> Box<AWKExpr> {
                Box::new(AWKExpr::FieldReference(record))
            },
        ),
    ))(input)
}

/// grouping (expr)
fn expr5(input: &str) -> IResult<&str, Box<AWKExpr>> {
    alt((
        value_or_name,
        delimited(char('('), delimited(wss, expr1, wss), char(')')),
    ))(input)
}

fn value_or_name(input: &str) -> IResult<&str, Box<AWKExpr>> {
    alt((
        map(parse_variable_name_expr, |e: AWKExpr| -> Box<AWKExpr> {
            Box::new(e)
        }),
        value,
    ))(input)
}

fn value(input: &str) -> IResult<&str, Box<AWKExpr>> {
    map(parse_value, |val: AWKVal| -> Box<AWKExpr> {
        Box::new(AWKExpr::Value(val))
    })(input)
}

#[test]
fn test_parse_expr() {
    let mut all = nom::combinator::all_consuming(parse_expr);

    assert!(all("123 - 444 * ( 555 - 666 ) - 2133").is_ok());
    assert_eq!(all("$(1*2)=\"hoge\""), all("$   ( 1 * 2 ) = \"hoge\""));
    assert_eq!(all("$1"), all("$                        1"));

    assert!(all(" 12 + 3").is_err());
    assert!(all("12 + 3 ").is_err());
}
