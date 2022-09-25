/*
 * file: item.rs
 * author: kota kato 2022
 * description:
 *   Parse item list
 */

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::char,
    combinator::{map,opt},
    multi::{many0, separated_list0},
    sequence::{delimited, tuple},
    IResult,
};

use crate::ast::{
    def::{AWKItem, AWKPattern, AWKPatternAction, AWKStat},
    stmt::parse_statement,
    util::*,
};

/*
 * action
 * pattern action
 * normal_pattern TODO
 */
pub fn parse_item(input: &str) -> IResult<&str, AWKItem> {
    alt((
        map(parse_action, |action: Vec<AWKStat>| {
            AWKItem::PatternAction(AWKPatternAction {
                pattern: AWKPattern::Always,
                action,
            })
        }),
        map(
            tuple((parse_pattern, wss, parse_action)),
            |(pattern, _, action): (AWKPattern, _, Vec<AWKStat>)| {
                AWKItem::PatternAction(AWKPatternAction { pattern, action })
            },
        ),
    ))(input)
}

fn parse_pattern(input: &str) -> IResult<&str, AWKPattern> {
    parse_special_pattern(input)
}

fn parse_special_pattern(input: &str) -> IResult<&str, AWKPattern> {
    let (input, tag) = alt((tag("BEGIN"), tag("END")))(input)?;
    let tag = match tag {
        "BEGIN" => AWKPattern::Begin,
        "END" => AWKPattern::End,
        _ => unreachable!(),
    };
    return Ok((input, tag));
}

fn parse_action(input: &str) -> IResult<&str, Vec<AWKStat>> {
    // 必ず;か\nを含み、任意の数の空白文字と;と\n
    fn parse_terminate(input: &str) -> IResult<&str, ()> {
        let (input, _) =
            tuple((wss, alt((char(';'), nl)), many0(alt((char(';'), ws, nl)))))(input)?;
        Ok((input, ()))
    }
    fn parse_statement_list(input: &str) -> IResult<&str, Vec<AWKStat>> {
        let (input, ret) = separated_list0(parse_terminate, parse_statement)(input)?;
        let (input, _) = opt(parse_terminate)(input)?;

        Ok((input, ret))
    }
    delimited(
        char('{'),
        map(
            tuple((opt(parse_terminate), parse_statement_list)),
            |(_, list): (_, Vec<AWKStat>)| list,
        ),
        char('}'),
    )(input)
}

#[test]
fn test_parse_item() {
    let a = parse_item("{}");
    let e = AWKItem::PatternAction(AWKPatternAction {
        pattern: AWKPattern::Always,
        action: vec![],
    });
    assert_eq!(Ok(("", e)), a);

    let a = parse_item("BEGIN{}");
    let e = AWKItem::PatternAction(AWKPatternAction {
        pattern: AWKPattern::Begin,
        action: vec![],
    });
    assert_eq!(Ok(("", e)), a);

    // white space -> OK NEWLINE -> NG
    assert!(parse_item("BEGIN {}").is_ok());
    assert!(parse_item("BEGIN \n{}").is_err());
}

#[test]
fn test_parse_action() {
    let expect = parse_action("{print(\"hoge\");1+2;print(23)}");

    assert!(expect.is_ok());
    assert_eq!(
        expect,
        parse_action(
            r#"{ ;;

                print("hoge");

                1+2

                ;
                ;

                ;


                print(23)
                ; }"#
        )
    );
}
