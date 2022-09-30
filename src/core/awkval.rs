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

    pub fn is_true(&self) -> bool {
        match self {
            AWKVal::Num(n) => n.clone() == 1.0,
            AWKVal::Str(s) => s.len() != 0,
            AWKVal::None => false,
        }
    }
}

impl AWKVal {
    // 多倍長整数演算
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
    pub fn module(&self, val: &AWKVal) -> AWKVal {
        AWKVal::Num(self.to_float() % val.to_float())
    }
    pub fn pow(&self, val: &AWKVal) -> AWKVal {
        AWKVal::Num(self.to_float().powf(val.to_float()))
    }
    pub fn not(&self) -> AWKVal {
        AWKVal::Num(if self.is_true() { 0.0 } else { 1.0 })
    }
    pub fn plus(&self) -> AWKVal {
        AWKVal::Num(self.to_float())
    }
    pub fn minus(&self) -> AWKVal {
        AWKVal::Num(self.to_float() * -1.0)
    }
    pub fn concat(&self, val: &AWKVal) -> AWKVal {
        AWKVal::Str(self.to_str() + &val.to_str())
    }
    pub fn and(&self, val: &AWKVal) -> AWKVal {
        // 短絡評価
        if !self.is_true() {
            return AWKVal::Num(0.0);
        };
        if !val.is_true() {
            return AWKVal::Num(0.0);
        };
        return AWKVal::Num(1.0);
    }
    pub fn or(&self, val: &AWKVal) -> AWKVal {
        // 短絡評価
        if self.is_true() {
            return AWKVal::Num(1.0);
        };
        if val.is_true() {
            return AWKVal::Num(1.0);
        };
        return AWKVal::Num(0.0);
    }
}
