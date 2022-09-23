/*
 * file: name.rs
 * author: kota kato 2022
 * description:
 *   parser variable name
 */

use crate::ast::def::AWKExpr;
use nom::{
    character::complete::one_of,
    combinator::{map_res, opt},
    multi::many0,
    sequence::tuple,
    IResult,
};

pub fn parse_name(input: &str) -> IResult<&str, AWKExpr> {
    map_res(
        tuple((
            one_of("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_"),
            opt(many0(one_of(
                "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_0123456789",
            ))),
        )),
        |(c, cs): (char, Option<Vec<char>>)| -> Result<AWKExpr, _> {
            let c = c.to_string();
            let cs = match cs {
                Some(cs) => cs.into_iter().collect(),
                None => "".to_string(),
            };
            let name = c + &cs;
            if !is_awk_reserved_name(&name) {
                return Ok(AWKExpr::Name(name));
            } else {
                return Err("Reserved Token");
            }
        },
    )(input)
}

pub fn is_awk_reserved_name(name: &str) -> bool {
    let list = [
        "BEGIN", "delete", "END", "function", "in", "printf", "break", "do", "exit", "getline",
        "next", "return", "continue", "else", "for", "if", "print", "while",
    ];
    list.iter().any(|n| n == &name)
}

#[test]
fn test_parse_name() {
    assert_eq!(
        parse_name("_unChi1233"),
        Ok(("", AWKExpr::Name("_unChi1233".to_string())))
    )
}

#[test]
fn test_is_awk_reserved_name() {
    assert_eq!(true, is_awk_reserved_name("BEGIN"));
    assert_eq!(false, is_awk_reserved_name("myvar"));
}
