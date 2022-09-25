/*
 * file: program.rs
 * author: kota kato 2022
 * description:
 *   program parser
 */

use crate::ast::{
    def::{AWKItem, AWKProgram},
    item::parse_item,
    util::*,
};
use nom::{
    character::complete::char,
    combinator::{map, opt},
    multi::separated_list0,
    sequence::{delimited, tuple},
    IResult,
};

pub fn parse_program(input: &str) -> IResult<&str, AWKProgram> {
    delimited(
        ws_nl_s,
        map(parse_item_list, |item_list| AWKProgram { item_list }),
        ws_nl_s,
    )(input)
}

// ITEMLIST := ITEM WSNLs TERMINATOR WSNLSs ITEM TERMINATOR
fn parse_item_list(input: &str) -> IResult<&str, Vec<AWKItem>> {
    let (input, ret) = separated_list0(delimited(ws_nl_s, char(';'), ws_nl_s), parse_item)(input)?;
    let (input, _) = tuple((ws_nl_s, opt(char(';'))))(input)?;

    Ok((input, ret))
}

#[test]
fn test_parse_program() {
    assert!(parse_program("  BEGIN{}  ").is_ok());
    assert!(parse_program("  BEGIN{};END{};  ").is_ok());
}

#[test]
fn test_parse_item_list() {
    assert!(parse_item_list("BEGIN{}").is_ok());
    assert!(parse_item_list("BEGIN{}  ;  \n\n\n END{} \n;").is_ok());
    assert!(parse_item_list("BEGIN{}  ;  \n\n\n END{}").is_ok());
    assert_eq!(
        parse_item_list("BEGIN{}  ;  \n\n\n END{}"),
        parse_item_list("BEGIN{};END{}")
    );
}
