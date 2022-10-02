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

enum CO {
    LT,  // <
    LET, // <=
    NE,  // !=
    EQ,  // == 
    GT,  // >
    GET, // >=
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
    // 比較のルール
    // 両方が数字 -> 数値として比較する
    // それ以外 -> 文字列に変換して比較する
    // POSIXの記述は誤りです
    //
    fn compbase(&self, val: &AWKVal, op: CO) -> AWKVal {
        let (left, right) = (self, val);
        AWKVal::Num(
            if match (left, right) {
                (AWKVal::Num(left), AWKVal::Num(right)) => {
                    match op {
                        CO::LT => left < right,
                        CO::LET => left <= right,
                        CO::NE => left != right,
                        CO::EQ => left == right,
                        CO::GT => left > right,
                        CO::GET => left <= right,
                    }
                },
                (_, _) => {
                    let (left, right) = (left.to_str(), right.to_str());
                    match op {
                        CO::LT => left < right,
                        CO::LET => left <= right,
                        CO::NE => left != right,
                        CO::EQ => left == right,
                        CO::GT => left > right,
                        CO::GET => left <= right,
                    }
                }
            } {
                1.0
            } else {
                0.0
            },
        )
    }
    // <
    pub fn lessthan(&self, val: &AWKVal) -> AWKVal {
        self.compbase(val, CO::LT)
    }
    pub fn lessequalthan(&self, val: &AWKVal) -> AWKVal {
        self.compbase(val, CO::LET)
    }
    pub fn notequal(&self, val: &AWKVal) -> AWKVal {
        self.compbase(val, CO::NE)
    }
    pub fn equal(&self, val: &AWKVal) -> AWKVal {
        self.compbase(val, CO::EQ)
    }
    pub fn greaterthan(&self, val: &AWKVal) -> AWKVal {
        self.compbase(val, CO::GT)
    }
    pub fn greaterequalthan(&self, val: &AWKVal) -> AWKVal {
        self.compbase(val, CO::GET)
    }
}
