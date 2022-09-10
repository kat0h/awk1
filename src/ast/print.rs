use nom::{
    branch::permutation, bytes::complete::tag, character::complete::char, combinator::map,
    multi::many0, sequence::delimited, IResult,
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
        delimited(char('('), parse_print_expr_list, char(')')),
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
