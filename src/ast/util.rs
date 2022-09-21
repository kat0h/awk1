/*
 * file: util.rs
 * author: nom
 * description:
 *   https://github.com/Geal/nom/blob/main/doc/nom_recipes.md#whitespace
 */

use nom::{character::complete::multispace0, error::ParseError, sequence::delimited, IResult};

/// A combinator that takes a parser `inner` and produces a parser that also consumes both leading and
/// trailing whitespace, returning the output of `inner`.
pub fn ws<'a, F: 'a, O, E: ParseError<&'a str>>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: FnMut(&'a str) -> IResult<&'a str, O, E>,
{
    delimited(multispace0, inner, multispace0)
}
