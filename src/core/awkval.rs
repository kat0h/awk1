/*
 * file: awkval.rs
 * author: kota kato 2022
 * description:
 *   implement for AWKVal
 */

use crate::ast::def::*;
use crate::ast::number::parse_number;

impl AWKVal {
    // AWKValue -> AWKNum / AWKStr
    pub fn to_str(&self) -> AWKStr {
        match self {
            AWKVal::Num(n) => n.to_string(),
            AWKVal::Str(s) => s.clone(),
            AWKVal::None => "".to_string(),
        }
    }

    pub fn to_float(&self) -> AWKFloat {
        match self {
            AWKVal::Num(n) => n.clone(),
            AWKVal::Str(s) => match parse_number(&s) {
                Ok((_, n)) => n,
                Err(_) => 0.0,
            },
            AWKVal::None => 0.0,
        }
    }
}

impl AWKVal {
    pub fn add(&self, val: &AWKVal) -> AWKVal {
        AWKVal::Num(self.to_float() + val.to_float())
    }
    pub fn sub(&self, val: &AWKVal) -> AWKVal {
        AWKVal::Num(self.to_float() - val.to_float())
    }
    pub fn mul(&self, val: &AWKVal) -> AWKVal {
        AWKVal::Num(self.to_float() * val.to_float())
    }
    pub fn div(&self, val: &AWKVal) -> AWKVal {
        AWKVal::Num(self.to_float() / val.to_float())
    }
}
