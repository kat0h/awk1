/*
 * file: util.rs
 * author: nom
 * description:
 *   https://github.com/Geal/nom/blob/main/doc/nom_recipes.md#whitespace
 */

use nom::{branch::alt, character::complete::char, multi::many0, IResult};

fn ws(input: &str) -> IResult<&str, char> {
    alt((char(' '), char('\t')))(input)
}

pub fn wss(input: &str) -> IResult<&str, Vec<char>> {
    many0(ws)(input)
}

pub fn nl(input: &str) -> IResult<&str, char> {
    alt((char('\n'), char('\r')))(input)
}

pub fn ws_nl_s(input: &str) -> IResult<&str, Vec<char>> {
    many0(alt((ws, nl)))(input)
}
