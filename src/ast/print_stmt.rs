/*
 * file: print.rs
 * author: kota kato 2022
 * description:
 *   print statement
 */

use nom::{
    branch::alt,
    character::complete::char,
    combinator::{map, map_res, opt},
    error::ErrorKind,
    multi::separated_list1,
    sequence::{delimited, tuple},
    IResult,
};

use crate::ast::{
    def::{AWKExpr, AWKPrint, AWKStat},
    expr::parse_expr,
    name::parse_name,
    util::*,
};

pub fn parse_print_stmt(input: &str) -> IResult<&str, AWKStat> {
    map(parse_print, |print: AWKPrint| -> AWKStat {
        AWKStat::Print(print)
    })(input)
}

// simple_print_statement
fn parse_print(input: &str) -> IResult<&str, AWKPrint> {
    let parse_print_tag = map_res(parse_name, |name: String| -> Result<&str, ErrorKind> {
        if &name == "print" {
            Ok("print")
        } else {
            Err(ErrorKind::MapRes)
        }
    });

    let parse_print_arguments = map(
        opt(alt((
            parse_print_expr_list,
            delimited(
                char('('),
                delimited(wss, parse_print_expr_list, wss),
                char(')'),
            ),
        ))),
        |expr: Option<Vec<Box<AWKExpr>>>| -> Vec<Box<AWKExpr>> {
            match expr {
                Some(expr) => expr,
                None => vec![],
            }
        },
    );

    // printと引数の間には空白文字のみ許可される
    let (input, (_, _, exprlist)) = tuple((parse_print_tag, wss, parse_print_arguments))(input)?;

    Ok((input, AWKPrint { exprlist }))
}

fn parse_print_expr_list(input: &str) -> IResult<&str, Vec<Box<AWKExpr>>> {
    // print_expr_list : print_expr
    //                 | print_expr_list ',' newline_opt print_expr
    // カンマのあとに改行が入ることが許可されるが、カンマの前は空白のみしか許可されない
    separated_list1(tuple((wss, char(','), ws_nl_s)), parse_expr)(input)
}

#[test]
fn test_parse_print_expr_list() {
    let e = vec![
        parse_expr("123").unwrap().1,
        parse_expr("\"hoge\"").unwrap().1,
    ];
    let a = parse_print_expr_list("123,\"hoge\"").unwrap().1;
    assert_eq!(e, a);
}

#[test]
fn test_parse_print() {
    assert_eq!(
        Ok((
            "",
            AWKPrint {
                exprlist: parse_print_expr_list("123,\"456\"").unwrap().1
            }
        )),
        parse_print("print(123,\"456\")")
    );
    assert_eq!(
        Ok(("", AWKPrint { exprlist: vec![] })),
        parse_print("print")
    );
    assert_eq!(
        Ok(("()", AWKPrint { exprlist: vec![] })),
        parse_print("print()")
    );

    let expect = parse_print("print(123,456)");

    let actual = parse_print(
        r#"print (   123,
    456     )"#,
    );
    assert_eq!(expect, actual);

    let mut all = nom::combinator::all_consuming(parse_print);
    assert!(all(r#"print
    ()"#)
    .is_err());
    assert!(all(r#"print (123
        ,3)"#)
    .is_err());
    assert!(all(r#"print (123,3
        )"#)
    .is_err());

    assert!(all("print (2+2)>3").is_ok());

    // パーサーが余計なスペースを処理していないかをチェック
    assert!(all("print 2 ").is_err());
    assert!(all(" print 2").is_err());
    assert!(all("print (2) ").is_err());
    assert!(all(" print (2)").is_err());
}
