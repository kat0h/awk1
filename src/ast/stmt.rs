/*
 * file: statement.rs
 * author: kota kato 2022
 * description:
 *   Parse statement
 */

use crate::ast::{
    def::{AWKExpr, AWKStat},
    expr::parse_expr,
    print_stmt::parse_print_stmt,
};
use nom::{branch::alt, combinator::map, IResult};

pub fn parse_statement(input: &str) -> IResult<&str, AWKStat> {
    alt((parse_expr_stmt, parse_print_stmt))(input)
}

pub fn parse_expr_stmt(input: &str) -> IResult<&str, AWKStat> {
    map(parse_expr, |e: Box<AWKExpr>| -> AWKStat {
        AWKStat::Expr(e)
    })(input)
}

#[test]
fn test_parse_statement() {
    assert_eq!(
        Ok(("", parse_print_stmt("print(123)").unwrap().1)),
        parse_statement("print(123)")
    );
}
