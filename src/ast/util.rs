/*
 * file: util.rs
 * author: nom
 * description:
 *   https://github.com/Geal/nom/blob/main/doc/nom_recipes.md#whitespace
 */

use nom::{
    branch::alt,
    character::complete::{char, multispace0},
    error::ParseError,
    multi::many0,
    sequence::delimited,
    IResult,
};

pub fn ws(input: &str) -> IResult<&str, char> {
    alt((char(' '), char('\t')))(input)
}

pub fn nl(input: &str) -> IResult<&str, char> {
    alt((char('\n'), char('\r')))(input)
}

pub fn ws_nl_s(input: &str) -> IResult<&str, Vec<char>> {
    many0(alt((ws, nl)))(input)
}

pub fn ws_right<'a, F: 'a, O, E: ParseError<&'a str>>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: FnMut(&'a str) -> IResult<&'a str, O, E>,
{
    delimited(multispace0, inner, multispace0)
}
