/*
 * file: statement.rs
 * author: kota kato 2022
 * description:
 *   Parse statement
 */

use crate::ast::{
    def::{AWKPrint, AWKStat},
    print_stmt::parse_print,
};
use nom::{combinator::map, IResult};

pub fn parse_statement(input: &str) -> IResult<&str, AWKStat> {
    map(parse_print, |p: AWKPrint| -> AWKStat { AWKStat::Print(p) })(input)
}

#[test]
fn test_parse_statement() {
    assert_eq!(
        Ok(("", AWKStat::Print(parse_print("print(123)").unwrap().1))),
        parse_statement("print(123)")
    );
}
