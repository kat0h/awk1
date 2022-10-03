/*
 * file: expr.rs
 * author: kota kato 2022
 * description:
 *   parsiong expression
 */

use crate::ast::{
    def::{AWKBinaryOperation, AWKExpr, AWKLval, AWKUnaryOperation, AWKVal},
    name::{parse_variable_name_expr, parse_variable_name_string},
    util::*,
    value::parse_value,
};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::char,
    combinator::{map, not},
    multi::many0,
    sequence::{delimited, tuple},
    IResult,
};

pub fn parse_expr(input: &str) -> IResult<&str, AWKExpr> {
    map(expr1, |expr| *expr)(input)
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
                    op: AWKBinaryOperation::Add,
                    left: Box::new(lval2awkexpr(&l)),
                    right: e,
                }),
            }),
            "-=" => Box::new(AWKExpr::Assign {
                lval: l.clone(),
                expr: Box::new(AWKExpr::BinaryOperation {
                    op: AWKBinaryOperation::Sub,
                    left: Box::new(lval2awkexpr(&l)),
                    right: e,
                }),
            }),
            "*=" => Box::new(AWKExpr::Assign {
                lval: l.clone(),
                expr: Box::new(AWKExpr::BinaryOperation {
                    op: AWKBinaryOperation::Mul,
                    left: Box::new(lval2awkexpr(&l)),
                    right: e,
                }),
            }),
            "/=" => Box::new(AWKExpr::Assign {
                lval: l.clone(),
                expr: Box::new(AWKExpr::BinaryOperation {
                    op: AWKBinaryOperation::Div,
                    left: Box::new(lval2awkexpr(&l)),
                    right: e,
                }),
            }),
            "^=" => Box::new(AWKExpr::Assign {
                lval: l.clone(),
                expr: Box::new(AWKExpr::BinaryOperation {
                    op: AWKBinaryOperation::Pow,
                    left: Box::new(lval2awkexpr(&l)),
                    right: e,
                }),
            }),
            "%=" => Box::new(AWKExpr::Assign {
                lval: l.clone(),
                expr: Box::new(AWKExpr::BinaryOperation {
                    op: AWKBinaryOperation::Mod,
                    left: Box::new(lval2awkexpr(&l)),
                    right: e,
                }),
            }),
            _ => unreachable!(),
        },
    );
    alt((assign, expr2))(input)
}

/// ||
fn expr2(input: &str) -> IResult<&str, Box<AWKExpr>> {
    let symbol = delimited(wss, tag("||"), wss);

    map(
        tuple((expr3, many0(tuple((symbol, expr3))))),
        |(expr, exprs): (Box<AWKExpr>, Vec<(&str, Box<AWKExpr>)>)| -> Box<AWKExpr> {
            // [1, 2, 3, 4] -> [[[1, 2], 3], 4]
            let mut i = expr;
            for j in exprs {
                i = Box::new(AWKExpr::BinaryOperation {
                    op: AWKBinaryOperation::Or,
                    left: i,
                    right: j.1,
                });
            }
            i
        },
    )(input)
}

/// &&
fn expr3(input: &str) -> IResult<&str, Box<AWKExpr>> {
    let symbol = delimited(wss, tag("&&"), wss);

    map(
        tuple((expr4, many0(tuple((symbol, expr4))))),
        |(expr, exprs): (Box<AWKExpr>, Vec<(&str, Box<AWKExpr>)>)| -> Box<AWKExpr> {
            // [1, 2, 3, 4] -> [[[1, 2], 3], 4]
            let mut i = expr;
            for j in exprs {
                i = Box::new(AWKExpr::BinaryOperation {
                    op: AWKBinaryOperation::And,
                    left: i,
                    right: j.1,
                });
            }
            i
        },
    )(input)
}

// 比較(複数個を繋げることはできません)
// LessThan,         // <
// LessEqualThan,    // <=
// NotEqual,         // !=
// Equal,            // ==
// GreaterThan,      // >
// GreaterEqualThan, // >=
fn expr4(input: &str) -> IResult<&str, Box<AWKExpr>> {
    let symbol = delimited(
        wss,
        alt((
            tag("<="),
            tag("!="),
            tag(">="),
            tag("=="),
            tag("<"),
            tag(">"),
        )),
        wss,
    );
    alt((
        map(
            tuple((expr5, symbol, expr5)),
            |(left, op, right): (Box<AWKExpr>, &str, Box<AWKExpr>)| -> Box<AWKExpr> {
                Box::new(AWKExpr::BinaryOperation {
                    op: match op {
                        "<" => AWKBinaryOperation::LessThan,
                        "<=" => AWKBinaryOperation::LessEqualThan,
                        "!=" => AWKBinaryOperation::NotEqual,
                        "==" => AWKBinaryOperation::Equal,
                        ">" => AWKBinaryOperation::GreaterThan,
                        ">=" => AWKBinaryOperation::GreaterEqualThan,
                        _ => unreachable!(),
                    },
                    left,
                    right,
                })
            },
        ),
        expr5,
    ))(input)
}

/// String concatenation
fn expr5(input: &str) -> IResult<&str, Box<AWKExpr>> {
    alt((
        map(
            tuple((expr6, wss, expr6)),
            |(left, _, right): (Box<AWKExpr>, _, Box<AWKExpr>)| {
                Box::new(AWKExpr::BinaryOperation {
                    op: AWKBinaryOperation::Cat,
                    left,
                    right,
                })
            },
        ),
        expr6,
    ))(input)
}

/// parse + -
fn expr6(input: &str) -> IResult<&str, Box<AWKExpr>> {
    let symbol = delimited(wss, alt((char('+'), char('-'))), wss);

    map(
        tuple((expr7, many0(tuple((symbol, expr7))))),
        |(expr, exprs): (Box<AWKExpr>, Vec<(char, Box<AWKExpr>)>)| -> Box<AWKExpr> {
            // [1, 2, 3, 4] -> [[[1, 2], 3], 4]
            let mut i = expr;
            for j in exprs {
                match j {
                    ('+', k) => {
                        i = Box::new(AWKExpr::BinaryOperation {
                            op: AWKBinaryOperation::Add,
                            left: i,
                            right: k,
                        });
                    }
                    ('-', k) => {
                        i = Box::new(AWKExpr::BinaryOperation {
                            op: AWKBinaryOperation::Sub,
                            left: i,
                            right: k,
                        })
                    }
                    _ => unreachable!(),
                }
            }
            i
        },
    )(input)
}

/// parse * /
fn expr7(input: &str) -> IResult<&str, Box<AWKExpr>> {
    let symbol = delimited(wss, alt((char('*'), char('/'), char('%'))), wss);

    map(
        tuple((expr8, many0(tuple((symbol, expr8))))),
        |(expr, exprs): (Box<AWKExpr>, Vec<(char, Box<AWKExpr>)>)| -> Box<AWKExpr> {
            let mut i = expr;
            for j in exprs {
                match j {
                    ('*', k) => {
                        i = Box::new(AWKExpr::BinaryOperation {
                            op: AWKBinaryOperation::Mul,
                            left: i,
                            right: k,
                        });
                    }
                    ('/', k) => {
                        i = Box::new(AWKExpr::BinaryOperation {
                            op: AWKBinaryOperation::Div,
                            left: i,
                            right: k,
                        })
                    }
                    ('%', k) => {
                        i = Box::new(AWKExpr::BinaryOperation {
                            op: AWKBinaryOperation::Mod,
                            left: i,
                            right: k,
                        })
                    }
                    _ => unreachable!(),
                }
            }
            i
        },
    )(input)
}

fn expr8(input: &str) -> IResult<&str, Box<AWKExpr>> {
    // ++ --が二文字連続している場合はErr
    let not2char = |c: char| map(tuple((char(c), not(char(c)))), |(c, _): (char, _)| c);
    let symbol = map(
        tuple((alt((char('!'), not2char('+'), not2char('-'))), wss)),
        |(s, _): (char, _)| s,
    );

    alt((
        map(
            tuple((symbol, expr8)),
            |(s, r): (char, Box<AWKExpr>)| -> Box<AWKExpr> {
                Box::new(AWKExpr::UnaryOperation {
                    expr: r,
                    op: match s {
                        '!' => AWKUnaryOperation::Not,
                        '+' => AWKUnaryOperation::Plus,
                        '-' => AWKUnaryOperation::Minus,
                        _ => unreachable!(),
                    },
                })
            },
        ),
        expr9,
    ))(input)
}

/// parse ^ 右結合
fn expr9(input: &str) -> IResult<&str, Box<AWKExpr>> {
    let symbol = delimited(wss, char('^'), wss);

    alt((
        map(
            tuple((
                expr10,
                map(tuple((symbol, expr9)), |(_, e): (_, Box<AWKExpr>)| e),
            )),
            |(l, r): (Box<AWKExpr>, Box<AWKExpr>)| {
                Box::new(AWKExpr::BinaryOperation {
                    op: AWKBinaryOperation::Pow,
                    left: l,
                    right: r,
                })
            },
        ),
        expr10,
    ))(input)
}

/// pre increment/decrement ++lval --lval
fn expr10(input: &str) -> IResult<&str, Box<AWKExpr>> {
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
    alt((incdec, expr11))(input)
}

/// post increment/decrement lval++ lval--
/// i++ - i みたいなパターン
// a----a a-----a a-- - -1とか
fn expr11(input: &str) -> IResult<&str, Box<AWKExpr>> {
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
    alt((incdec, expr12))(input)
}

/// parse field reference $expr
fn expr12(input: &str) -> IResult<&str, Box<AWKExpr>> {
    let field_reference = tuple((char('$'), wss, expr13));
    alt((
        expr13,
        map(
            field_reference,
            |(_, _, record): (char, _, Box<AWKExpr>)| -> Box<AWKExpr> {
                Box::new(AWKExpr::Field(record))
            },
        ),
    ))(input)
}

/// grouping (expr)
fn expr13(input: &str) -> IResult<&str, Box<AWKExpr>> {
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
    let val = map(parse_variable_name_string, AWKLval::Name);
    let field = map(tuple((char('$'), wss, expr13)), |(_, _, expr)| {
        AWKLval::Field(expr)
    });
    alt((val, field))(input)
}

#[test]
fn test_parse_expr() {
    fn s(e: &AWKExpr) -> String {
        match e {
            AWKExpr::Value(v) => match v {
                AWKVal::Num(n) => n.to_string(),
                AWKVal::Str(s) => format!("{:?}", s.clone()),
                AWKVal::None => "None".to_string(),
            },
            AWKExpr::Name(s) => s.to_string(),
            AWKExpr::BinaryOperation { op, left, right } => {
                let op = match op {
                    AWKBinaryOperation::Add => "+",
                    AWKBinaryOperation::Sub => "-",
                    AWKBinaryOperation::Mul => "*",
                    AWKBinaryOperation::Div => "/",
                    AWKBinaryOperation::Pow => "^",
                    AWKBinaryOperation::Mod => "%",
                    AWKBinaryOperation::Cat => "..",
                    AWKBinaryOperation::And => "&&",
                    AWKBinaryOperation::Or => "||",
                    AWKBinaryOperation::LessThan => "<",
                    AWKBinaryOperation::LessEqualThan => "<=",
                    AWKBinaryOperation::NotEqual => "!=",
                    AWKBinaryOperation::Equal => "==",
                    AWKBinaryOperation::GreaterThan => ">",
                    AWKBinaryOperation::GreaterEqualThan => ">=",
                };
                format!("({} {} {})", op, &s(left), &s(right))
            }
            AWKExpr::Field(f) => format!("($ {})", &s(f)),
            AWKExpr::Assign { lval, expr } => {
                let lval = match lval {
                    AWKLval::Name(s) => s.clone(),
                    AWKLval::Field(e) => format!("($ {})", s(e)),
                };
                let expr = &s(expr);
                format!("(setq {} {})", lval, expr)
            }
            AWKExpr::IncDec {
                is_post,
                is_inc,
                lval,
            } => {
                let is_post = if *is_post { "post" } else { "pre" };
                let is_inc = if *is_inc { "++" } else { "--" };
                let lval = match lval {
                    AWKLval::Name(s) => s.clone(),
                    AWKLval::Field(e) => format!("($ {})", s(e)),
                };
                format!("({}{} {})", is_post, is_inc, lval)
            }
            AWKExpr::UnaryOperation { op, expr } => {
                let op = match op {
                    AWKUnaryOperation::Not => "!",
                    AWKUnaryOperation::Plus => "+",
                    AWKUnaryOperation::Minus => "-",
                };
                format!("({} {})", op, &s(expr))
            }
        }
    }
    let mut all = nom::combinator::all_consuming(parse_expr);

    let testcase = [
        (r#"(+ 1 2)"#, r#"1+2"#),
        (r#"(setq ($ (* 1 2)) "hoge")"#, r#"$(1*2)="hoge""#),
        (r#""hoge\n""#, r#""hoge\n""#),
        (
            r#"(- (+ (- (- 123 (* 444 (- 555 666))) (% 12 4)) (pre-- a)) (setq a (+ a 4)))"#,
            r#"123 - 444 * ( 555 - 666 ) - 12 % 4 + -- a - ( a += 4 )"#,
        ),
        (
            r#"(- (+ (- (- 123 (* 444 (- 555 666))) (% 12 4)) (pre-- a)) (setq a (+ a 4)))"#,
            r#"123-444*(555-666)-12%4+--a-(a+=4)"#,
        ),
        (r#"($ 1)"#, r#"$  1"#),
        // (r#""#, r#""#),
    ];

    for testcase in testcase.iter() {
        let expect = testcase.0;
        let actual = match all(testcase.1) {
            Ok(o) => s(&o.1),
            Err(e) => e.to_string(),
        };
        assert_eq!(expect, actual);
    }

    assert!(all(" 12 + 3").is_err());
    assert!(all("12 + 3 ").is_err());
}
