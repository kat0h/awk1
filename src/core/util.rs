/*
 * file: util.rs
 * author: kota kato 2022
 * description:
 *   Utility function
 */

use crate::ast::def::*;

// AWKValue -> AWKNum / AWKStr
pub fn to_awkstr(value: AWKVal) -> AWKStr {
    match value {
        AWKVal::Num(n) => n.to_string(),
        AWKVal::Str(s) => s.clone(),
    }
}

pub fn to_awknum(value: AWKVal) -> AWKFloat {
    use crate::ast::number::parse_number;
    match value {
        AWKVal::Num(n) => n,
        AWKVal::Str(s) => match parse_number(&s) {
            Ok((_, n)) => n,
            Err(_) => 0.0,
        },
    }
}
