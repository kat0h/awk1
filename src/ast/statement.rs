/*
 * file: statement.rs
 * author: kota kato 2022
 * description:
 *   Parse statement
 */

use crate::ast::{
    def::{AWKPrint, AWKStatement},
    print_statement::parse_print,
};
use nom::{combinator::map, IResult};

pub fn parse_statement(input: &str) -> IResult<&str, AWKStatement> {
    map(parse_print, |p: AWKPrint| -> AWKStatement {
        AWKStatement::Print(p)
    })(input)
}

#[test]
fn test_parse_statement() {
    assert_eq!(
        Ok((
            "",
            AWKStatement::Print(parse_print("print(123)").unwrap().1)
        )),
        parse_statement("print(123)")
    );
}
