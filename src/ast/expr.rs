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
    bytes::complete::tag,
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
    // cloneを排除する
    fn lval2awkexpr(lval: &AWKLval) -> AWKExpr {
        match lval {
            AWKLval::Name(n) => AWKExpr::Name(n.to_string()),
            AWKLval::Field(e) => AWKExpr::Field(e.clone()),
        }
    }

    let assign = map(
        tuple((
            lval,
            delimited(
                wss,
                alt((
                    tag("="),
                    tag("+="),
                    tag("-="),
                    tag("*="),
                    tag("/="),
                    tag("^="),
                    tag("%="),
                )),
                wss,
            ),
            expr1,
        )),
        |(l, assigntype, e)| match assigntype {
            "=" => Box::new(AWKExpr::Assign { lval: l, expr: e }),
            "+=" => Box::new(AWKExpr::Assign {
                lval: l.clone(),
                expr: Box::new(AWKExpr::BinaryOperation {
                    op: AWKOperation::Add,
                    left: Box::new(lval2awkexpr(&l)),
                    right: e,
                }),
            }),
            "-=" => Box::new(AWKExpr::Assign {
                lval: l.clone(),
                expr: Box::new(AWKExpr::BinaryOperation {
                    op: AWKOperation::Sub,
                    left: Box::new(lval2awkexpr(&l)),
                    right: e,
                }),
            }),
            "*=" => Box::new(AWKExpr::Assign {
                lval: l.clone(),
                expr: Box::new(AWKExpr::BinaryOperation {
                    op: AWKOperation::Mul,
                    left: Box::new(lval2awkexpr(&l)),
                    right: e,
                }),
            }),
            "/=" => Box::new(AWKExpr::Assign {
                lval: l.clone(),
                expr: Box::new(AWKExpr::BinaryOperation {
                    op: AWKOperation::Div,
                    left: Box::new(lval2awkexpr(&l)),
                    right: e,
                }),
            }),
            "^=" => Box::new(AWKExpr::Assign {
                lval: l.clone(),
                expr: Box::new(AWKExpr::BinaryOperation {
                    op: AWKOperation::Pow,
                    left: Box::new(lval2awkexpr(&l)),
                    right: e,
                }),
            }),
            "%=" => Box::new(AWKExpr::Assign {
                lval: l.clone(),
                expr: Box::new(AWKExpr::BinaryOperation {
                    op: AWKOperation::Mod,
                    left: Box::new(lval2awkexpr(&l)),
                    right: e,
                }),
            }),
            _ => unreachable!(),
        },
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
    let symbol = delimited(wss, alt((char('*'), char('/'), char('%'))), wss);

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
                    ('%', k) => {
                        i = Box::new(AWKExpr::BinaryOperation {
                            op: AWKOperation::Mod,
                            left: i,
                            right: k,
                        })
                    }
                    _ => unreachable!(),
                }
            }
            return i;
        },
    )(input)
}

/// parse ^ 右結合
fn expr4(input: &str) -> IResult<&str, Box<AWKExpr>> {
    let symbol = delimited(wss, char('^'), wss);

    alt((
        map(
            tuple((
                expr5,
                map(tuple((symbol, expr4)), |(_, e): (_, Box<AWKExpr>)| e),
            )),
            |(l, r): (Box<AWKExpr>, Box<AWKExpr>)| {
                Box::new(AWKExpr::BinaryOperation {
                    op: AWKOperation::Pow,
                    left: l,
                    right: r,
                })
            },
        ),
        expr5,
    ))(input)
}

fn expr5(input: &str) -> IResult<&str, Box<AWKExpr>> {
    let is_inc = map(
        tuple((alt((tag("++"), tag("--"))), wss)),
        |(symbol, _): (&str, _)| match symbol {
            "++" => true,
            "--" => false,
            _ => unreachable!(),
        },
    );
    let incdec = map(tuple((is_inc, lval)), |(is_inc, lval): (bool, AWKLval)| {
        Box::new(AWKExpr::IncDec {
            is_inc,
            is_post: false,
            lval,
        })
    });
    alt((incdec, expr6))(input)
}

/// post increment/decrement lval++ lval--
/// i++ - i みたいなパターン
// a----a a-----a a-- - -1とか
fn expr6(input: &str) -> IResult<&str, Box<AWKExpr>> {
    let is_inc = map(
        tuple((wss, alt((tag("++"), tag("--"))))),
        |(_, symbol): (_, &str)| match symbol {
            "++" => true,
            "--" => false,
            _ => unreachable!(),
        },
    );
    let incdec = map(tuple((lval, is_inc)), |(lval, is_inc): (AWKLval, bool)| {
        Box::new(AWKExpr::IncDec {
            is_inc,
            is_post: true,
            lval,
        })
    });
    alt((incdec, expr7))(input)
}

/// parse field reference $expr
fn expr7(input: &str) -> IResult<&str, Box<AWKExpr>> {
    let field_reference = tuple((char('$'), wss, expr8));
    alt((
        expr8,
        map(
            field_reference,
            |(_, _, record): (char, _, Box<AWKExpr>)| -> Box<AWKExpr> {
                Box::new(AWKExpr::Field(record))
            },
        ),
    ))(input)
}

/// grouping (expr)
fn expr8(input: &str) -> IResult<&str, Box<AWKExpr>> {
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

fn lval(input: &str) -> IResult<&str, AWKLval> {
    let val = map(parse_variable_name_string, |name| AWKLval::Name(name));
    let field = map(tuple((char('$'), wss, expr8)), |(_, _, expr)| {
        AWKLval::Field(expr)
    });
    alt((val, field))(input)
}

#[test]
fn test_parse_expr() {
    let mut all = nom::combinator::all_consuming(parse_expr);

    assert!(
        all("123 - 444 * ( 555 - 666 ) - 2133 % 1024 + 45 ^ 4 * ( a += 45 - a-- - 3) + ++a")
            .is_ok()
    );
    assert_eq!(all("$(1*2)=\"hoge\""), all("$   ( 1 * 2 ) = \"hoge\""));
    assert_eq!(all("$1"), all("$                        1"));

    assert!(all(" 12 + 3").is_err());
    assert!(all("12 + 3 ").is_err());
}
