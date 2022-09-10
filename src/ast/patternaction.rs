/* 
 * file: paction.rs
 * author: kota kato 2020
 * description:
 *   Parse awk program that is composed pattern and action statement
 */

/*
 * Action
 * pattern { action } ...
 */

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, multispace0},
    combinator::{map, opt},
    sequence::delimited,
    IResult,
};

use crate::ast::def::{
    AWKPattern,
    AWKAction,
    AWKPatternAction,
};

// BEGIN / END / nothing
pub fn parse_paction(input: &str) -> IResult<&str, AWKPatternAction> {
    let (input, pattern) = map(
        delimited(
            multispace0,
            opt(alt((tag("BEGIN"), tag("END")))),
            multispace0,
        ),
        |pattern: Option<&str>| -> AWKPattern {
            match pattern {
                Some("BEGIN") => AWKPattern::BEGIN,
                Some("END") => AWKPattern::END,
                _ => AWKPattern::Always,
            }
        },
    )(input)?;
    let (input, action) = map(
        delimited(
            char('{'),
            delimited(multispace0, tag("print"), multispace0),
            char('}'),
        ),
        |action: &str| -> AWKAction {
            AWKAction {
                statement: action.to_string(),
            }
        },
    )(input)?;
    return Ok((input, AWKPatternAction { pattern, action }));
}

#[test]
fn test_parse_string() {
    assert_eq!(
        Ok((
            "",
            AWKPatternAction {
                pattern: AWKPattern::BEGIN,
                action: AWKAction {
                    statement: "print".to_string()
                }
            }
        )),
        parse_paction("        BEGIN { print        }")
    );
    assert_eq!(
        Ok((
            "",
            AWKPatternAction {
                pattern: AWKPattern::Always,
                action: AWKAction {
                    statement: "print".to_string()
                }
            }
        )),
        parse_paction("     { print        }")
    );
}
