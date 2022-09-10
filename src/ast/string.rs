/* 
 * file: string.rs
 * author: kota kato 2020
 * description:
 *   Parse string literal that is delimited by " character
 */

#![allow(dead_code)]

use crate::ast::def::AWKString;

use nom::{
    branch::alt,
    bytes::complete::escaped_transform,
    character::complete::{char, none_of},
    combinator::{map, value},
    sequence::delimited,
    IResult,
};

pub fn parse_string(input: &str) -> IResult<&str, AWKString> {
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
        |str: String| -> AWKString { AWKString { value: str.clone() } },
    )(input)
}

#[test]
fn test_parse_string() {
    assert_eq!(
        Ok(("", AWKString { value: "TEST \n \"THE\" \\ World!!!".to_string() })),
        parse_string("\"TEST \\n \\\"THE\\\" \\\\ World!!!\"")
    )
}
