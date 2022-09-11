/*
 * file: program.rs
 * author: kota kato 2022
 * description:
 *   program parser
 */

use crate::ast::{
    def::{AWKItem, AWKProgram},
    item::parse_item,
};
use nom::{
    character::complete::char,
    combinator::{all_consuming, map},
    multi::separated_list0,
    IResult,
};

pub fn parse_program(input: &str) -> IResult<&str, AWKProgram> {
    all_consuming(map(parse_item_list, |item_list| AWKProgram { item_list }))(input)
}

fn parse_item_list(input: &str) -> IResult<&str, Vec<AWKItem>> {
    // TODO ; -> terminator
    separated_list0(char(';'), parse_item)(input)
}
