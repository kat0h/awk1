#![allow(dead_code)]

use nom::{
    branch::alt,
    bytes::complete::escaped_transform,
    character::complete::{char, none_of},
    combinator::{map, value},
    sequence::delimited,
    IResult,
};

#[derive(Debug, PartialEq)]
pub struct AWKSTRING {
    value: String,
}

pub fn parse_string(input: &str) -> IResult<&str, AWKSTRING> {
    map(
        delimited(
            char('\"'),
            escaped_transform(
                none_of("\"\\"),
                '\\',
                alt((
                    value('\\', char('\\')),
                    value('\"', char('\"')),
                    // value('\a', char('a')),
                    // value('\b', char('b')),
                    // value('\f', char('f')),
                    value('\n', char('n')),
                    value('\r', char('r')),
                    value('\t', char('t')),
                    // value('\v'. char('v')),
                )),
            ),
            char('\"'),
        ),
        |str: String| -> AWKSTRING { AWKSTRING { value: str.clone() } },
    )(input)
}

#[test]
fn test_parse_string() {
    assert_eq!(
        Ok(("", AWKSTRING { value: "TEST \n \"THE\" \\ World!!!".to_string() })),
        parse_string("\"TEST \\n \\\"THE\\\" \\\\ World!!!\"")
    )
}
