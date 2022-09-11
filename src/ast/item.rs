/*
 * file: item.rs
 * author: kota kato 2022
 * description:
 *   Parse item list
 */

use nom::{
    branch::{alt, permutation},
    bytes::complete::tag,
    character::complete::char,
    combinator::map,
    sequence::tuple,
    IResult,
};

use crate::ast::def::{AWKAction, AWKItem, AWKPattern, AWKPatternAction};

/*
 * action
 * pattern action
 * normal_pattern TODO
 */
pub fn parse_item(input: &str) -> IResult<&str, AWKItem> {
    alt((
        map(parse_action, |action: AWKAction| {
            AWKItem::AWKPatternAction(AWKPatternAction {
                pattern: AWKPattern::Always,
                action,
            })
        }),
        map(
            permutation((parse_pattern, parse_action)),
            |(pattern, action): (AWKPattern, AWKAction)| {
                AWKItem::AWKPatternAction(AWKPatternAction { pattern, action })
            },
        ),
    ))(input)
}

fn parse_action(input: &str) -> IResult<&str, AWKAction> {
    let (input, _) = tuple((char('{'), char('}')))(input)?;
    Ok((input, AWKAction {}))
}

fn parse_pattern(input: &str) -> IResult<&str, AWKPattern> {
    parse_special_pattern(input)
}

#[allow(dead_code)]
fn parse_normal_pattern(_input: &str) -> IResult<&str, AWKPattern> {
    unimplemented!()
}

fn parse_special_pattern(input: &str) -> IResult<&str, AWKPattern> {
    let (input, tag) = alt((tag("BEGIN"), tag("END")))(input)?;
    let tag = match tag {
        "BEGIN" => AWKPattern::Begin,
        _ => AWKPattern::End,
    };
    return Ok((input, tag));
}

#[test]
fn test_parse_item() {
    let a = parse_item("{}");
    let e = AWKItem::AWKPatternAction(AWKPatternAction {
        pattern: AWKPattern::Always,
        action: AWKAction {},
    });
    assert_eq!(Ok(("", e)), a);

    let a = parse_item("BEGIN{}");
    let e = AWKItem::AWKPatternAction(AWKPatternAction {
        pattern: AWKPattern::Begin,
        action: AWKAction {},
    });
    assert_eq!(Ok(("", e)), a);
}
