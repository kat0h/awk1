/*
 * file: statement.rs
 * author: kota kato 2022
 * description:
 *   Parse statement
 */

use crate::ast::{
    def::{AWKExpr, AWKStat},
    expr::parse_expr,
    item::parse_action,
    print_stmt::parse_print_stmt,
};
use nom::{branch::alt, combinator::map, IResult};

pub fn parse_statement(input: &str) -> IResult<&str, AWKStat> {
    alt((parse_action_stmt, parse_print_stmt, parse_expr_stmt))(input)
}

// expr(式)はステートメントとしても扱うことができます」
fn parse_expr_stmt(input: &str) -> IResult<&str, AWKStat> {
    map(parse_expr, |e: Box<AWKExpr>| -> AWKStat {
        AWKStat::Expr(e)
    })(input)
}

fn parse_action_stmt(input: &str) -> IResult<&str, AWKStat> {
    map(parse_action, |e: Vec<AWKStat>| -> AWKStat {
        AWKStat::Action(e)
    })(input)
}

#[test]
fn test_parse_statement() {
    assert_eq!(
        Ok(("", parse_print_stmt("print(123)").unwrap().1)),
        parse_statement("print(123)")
    );

    let mut all = nom::combinator::all_consuming(parse_statement);
    assert!(all("{{{}}}").is_ok());
}
