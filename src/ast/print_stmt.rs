/*
 * file: print.rs
 * author: kota kato 2022
 * description:
 *   print statement
 */

use nom::{
    branch::alt,
    character::complete::char,
    combinator::{map, map_res, opt},
    error::ErrorKind,
    multi::separated_list1,
    sequence::{delimited, tuple},
    IResult,
};

use crate::ast::{
    def::{AWKExpr, AWKPrint, AWKStat},
    expr::parse_expr,
};

use super::name::parse_name;

pub fn parse_print_stmt(input: &str) -> IResult<&str, AWKStat> {
    map(parse_print, |print: AWKPrint| -> AWKStat {
        AWKStat::Print(print)
    })(input)
}

// simple_print_statement
fn parse_print(input: &str) -> IResult<&str, AWKPrint> {
    let (input, (_, exprlist)) = tuple((
        map_res(parse_name, |name: String| -> Result<&str, ErrorKind> {
            if &name == "print" {
                Ok("print")
            } else {
                Err(ErrorKind::MapRes)
            }
        }),
        map(
            opt(alt((
                delimited(char('('), parse_print_expr_list, char(')')),
                parse_print_expr_list,
            ))),
            |expr: Option<Vec<Box<AWKExpr>>>| -> Vec<Box<AWKExpr>> {
                match expr {
                    Some(expr) => expr,
                    None => vec![],
                }
            },
        ),
    ))(input)?;

    Ok((input, AWKPrint { exprlist }))
}

fn parse_print_expr_list(input: &str) -> IResult<&str, Vec<Box<AWKExpr>>> {
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
        Ok(("", AWKPrint { exprlist: vec![] })),
        parse_print("print")
    );
    assert_eq!(
        Ok(("()", AWKPrint { exprlist: vec![] })),
        parse_print("print()")
    );
}
