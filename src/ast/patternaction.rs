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
    character::complete::char,
    combinator::{map, opt},
    sequence::delimited,
    IResult,
};

use crate::ast::{
    def::{AWKAction, AWKPattern, AWKPatternAction, AWKPrint},
    print::parse_print,
};

// BEGIN / END / nothing
pub fn parse_paction(input: &str) -> IResult<&str, AWKPatternAction> {
    let (input, pattern) = map(
        opt(alt((tag("BEGIN"), tag("END")))),
        |pattern: Option<&str>| -> AWKPattern {
            match pattern {
                Some("BEGIN") => AWKPattern::BEGIN,
                Some("END") => AWKPattern::END,
                _ => AWKPattern::Always,
            }
        },
    )(input)?;
    // parse action
    let (input, action) = map(
        delimited(char('{'), parse_print, char('}')),
        |action: AWKPrint| -> AWKAction { AWKAction { statement: action } },
    )(input)?;
    return Ok((input, AWKPatternAction { pattern, action }));
}

#[test]
fn test_parse_patternaction() {
    assert_eq!(
        Ok((
            "",
            AWKPatternAction {
                pattern: AWKPattern::BEGIN,
                action: AWKAction {
                    statement: AWKPrint {
                        expr: crate::ast::expr::parse_expr("132").unwrap().1
                    }
                }
            }
        )),
        parse_paction("BEGIN{print 132}")
    );
}
