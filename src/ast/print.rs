use nom::{
    bytes::complete::tag,
    IResult,
};

use crate::ast::{def::AWKPrint, expr::parse_expr};

// print Expr
// print (Expr)
// print

pub fn parse_print(input: &str) -> IResult<&str, AWKPrint> {
    let (input, _) = tag("print ")(input)?;
    let (input, expr) = parse_expr(input)?;
    Ok((input, AWKPrint { expr }))
}
