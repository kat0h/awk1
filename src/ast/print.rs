use nom::{
    branch::permutation,
    bytes::complete::tag,
    character::complete::char,
    combinator::{map, opt},
    multi::many0,
    sequence::delimited,
    IResult,
};

use crate::ast::{def::AWKPrint, expr::parse_expr};

use super::def::AWKExpr;

fn parse_print_expr_list(input: &str) -> IResult<&str, Vec<AWKExpr>> {
    map(
        permutation((
            parse_expr,
            many0(map(
                permutation((char(','), parse_expr)),
                |(_, expr): (char, AWKExpr)| -> AWKExpr { expr },
            )),
        )),
        |(expr, exprs): (AWKExpr, Vec<AWKExpr>)| -> Vec<AWKExpr> {
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
            opt(delimited(char('('), parse_print_expr_list, char(')'))),
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
        Ok(("", AWKPrint {
            exprlist: parse_print_expr_list("123,\"456\"").unwrap().1
        })),
        parse_print("print(123,\"456\")")
    );
    assert_eq!(
        Ok(("", AWKPrint {
            exprlist: vec![]
        })),
        parse_print("print")
    );
    assert_eq!(
        Ok(("()", AWKPrint {
            exprlist: vec![]
        })),
        parse_print("print()")
    );
}
