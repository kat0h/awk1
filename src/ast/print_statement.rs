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
    multi::separated_list1,
    sequence::delimited,
    IResult,
};

use crate::ast::{def::AWKPrint, expr::parse_expr};

use super::def::AWKExpr;

// simple_print_statement
pub fn parse_print(input: &str) -> IResult<&str, AWKPrint> {
    let (input, (_, exprlist)) = permutation((
        tag("print"),
        map(
            opt(alt((
                delimited(char('('), parse_print_expr_list, char(')')),
                parse_print_expr_list,
            ))),
            |expr: Option<Vec<AWKExpr>>| -> Vec<AWKExpr> {
                match expr {
                    Some(expr) => expr,
                    None => vec![],
                }
            },
        ),
    ))(input)?;

    Ok((input, AWKPrint { exprlist }))
}

fn parse_print_expr_list(input: &str) -> IResult<&str, Vec<AWKExpr>> {
    separated_list1(char(','), parse_expr)(input)
}

#[test]
fn test_parse_print_expr_list() {
    let e = vec![
        parse_expr("123").unwrap().1,
        parse_expr("\"hoge\"").unwrap().1,
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
