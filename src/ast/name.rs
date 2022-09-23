/*
 * file: name.rs
 * author: kota kato 2022
 * description:
 *   parser variable name
 */

use crate::ast::def::AWKExpr;
use nom::{
    character::complete::one_of,
    combinator::{map, opt},
    multi::many0,
    sequence::tuple,
    IResult,
};

pub fn parse_name(input: &str) -> IResult<&str, AWKExpr> {
    map(
        tuple((
            one_of("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_"),
            opt(many0(one_of(
                "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_0123456789",
            ))),
        )),
        |(c, cs): (char, Option<Vec<char>>)| -> AWKExpr {
            let c = c.to_string();
            let cs = match cs {
                Some(cs) => cs.into_iter().collect(),
                None => "".to_string(),
            };
            AWKExpr::Name(c + &cs)
        },
    )(input)
}

#[test]
fn test_parse_name() {
    assert_eq!(
        parse_name("_unChi1233"),
        Ok(("", AWKExpr::Name("_unChi1233".to_string())))
    )
}
