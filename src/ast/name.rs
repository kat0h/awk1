/*
 * file: name.rs
 * author: kota kato 2022
 * description:
 *   parser variable name
 */

use crate::ast::def::AWKExpr;
use nom::{
    character::complete::one_of,
    combinator::{map, map_res, opt},
    error::ErrorKind,
    multi::many0,
    sequence::tuple,
    IResult,
};

// TODO: Refactor
/// AWKの変数名をパース。予約語を考慮。AWKExprとして返す
pub fn parse_variable_name_expr(input: &str) -> IResult<&str, AWKExpr> {
    map_res(parse_name, |s: String| -> Result<AWKExpr, _> {
        if !is_awk_reserved_name(&s) {
            return Ok(AWKExpr::Name(s));
        } else {
            return Err(ErrorKind::MapRes);
        }
    })(input)
}

/// AWKの変数名をパース
/// 予約語に含まれている場合Errを返す
pub fn parse_variable_name_string(input: &str) -> IResult<&str, String> {
    map_res(parse_name, |s: String| -> Result<String, _> {
        if !is_awk_reserved_name(&s) {
            return Ok(s);
        } else {
            return Err(ErrorKind::MapRes);
        }
    })(input)
}

/// AWKの名前をパース 予約語は考慮しません
pub fn parse_name(input: &str) -> IResult<&str, String> {
    map(
        tuple((
            one_of("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_"),
            opt(many0(one_of(
                "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_0123456789",
            ))),
        )),
        |(c, cs): (char, Option<Vec<char>>)| -> String {
            let c = c.to_string();
            let cs = match cs {
                Some(cs) => cs.into_iter().collect(),
                None => "".to_string(),
            };
            c + &cs
        },
    )(input)
}

/// 名前がAWKの予約語に含まれているかを判定
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
        parse_variable_name_expr("_unChi1233"),
        Ok(("", AWKExpr::Name("_unChi1233".to_string())))
    )
}

#[test]
fn test_is_awk_reserved_name() {
    assert_eq!(true, is_awk_reserved_name("BEGIN"));
    assert_eq!(false, is_awk_reserved_name("myvar"));
}
