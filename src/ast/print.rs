/*
 * file: print.rs
 * author: kota kato 2022
 * description:
 *   print statement
 */

use nom::{
    branch::{alt, permutation},
    bytes::complete::tag,
    character::complete::char,
    combinator::{map, opt},
    multi::many0,
    sequence::delimited,
    IResult,
};

use crate::ast::{def::AWKPrint, print_expr::parse_non_unary_print_expr};

use super::def::AWKNonUnaryPrintExpr;

fn parse_print_expr_list(input: &str) -> IResult<&str, Vec<AWKNonUnaryPrintExpr>> {
    map(
        permutation((
            parse_non_unary_print_expr,
            many0(map(
                permutation((char(','), parse_non_unary_print_expr)),
                |(_, expr): (char, AWKNonUnaryPrintExpr)| -> AWKNonUnaryPrintExpr { expr },
            )),
        )),
        |(expr, exprs): (AWKNonUnaryPrintExpr, Vec<AWKNonUnaryPrintExpr>)| -> Vec<AWKNonUnaryPrintExpr> {
            let mut exprlist = vec![expr];
            for e in exprs.iter() {
                // ここのcloneを消す
                exprlist.push(e.clone());
            }
            exprlist
        },
    )(input)
}

// fn parse_multiple_expr_list(input: &str) -> IResult<&str, ()> {}

// simple_print_statement
pub fn parse_print(input: &str) -> IResult<&str, AWKPrint> {
    // print(Expr,Expr,Expr)
    let (input, (_, exprlist)) = permutation((
        tag("print"),
        map(
            opt(alt((
                delimited(char('('), parse_print_expr_list, char(')')),
                parse_print_expr_list,
            ))),
            |expr: Option<Vec<AWKNonUnaryPrintExpr>>| -> Vec<AWKNonUnaryPrintExpr> {
                match expr {
                    Some(expr) => expr,
                    None => vec![],
                }
            },
        ),
    ))(input)?;

    Ok((input, AWKPrint { exprlist }))
}

#[test]
fn test_parse_print_expr_list() {
    let e = vec![
        parse_non_unary_print_expr("123").unwrap().1,
        parse_non_unary_print_expr("\"hoge\"").unwrap().1,
    ];
    let a = parse_print_expr_list("123,\"hoge\"").unwrap().1;
    assert_eq!(e, a);
}

#[test]
fn test_parse_print() {
    assert_eq!(
        Ok((
            "",
            AWKPrint {
                exprlist: parse_print_expr_list("123,\"456\"").unwrap().1
            }
        )),
        parse_print("print(123,\"456\")")
    );
    assert_eq!(
        Ok((
            "",
            AWKPrint {
                exprlist: parse_print_expr_list("123,\"456\"").unwrap().1
            }
        )),
        parse_print("print123,\"456\"")
    );
    assert_eq!(
        Ok(("", AWKPrint { exprlist: vec![] })),
        parse_print("print")
    );
    assert_eq!(
        Ok(("()", AWKPrint { exprlist: vec![] })),
        parse_print("print()")
    );
}
