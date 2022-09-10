/*
 * file: number.rs
 * author: kota kato 2022
 * description:
 *   Parse number literal
 */

#[derive(Debug, PartialEq, Clone)]
pub enum AWKNonUnaryPrintExpr {
    AWKNumber(AWKNumber),
    AWKString(AWKString),
}

#[derive(Debug, PartialEq, Clone)]
pub struct AWKString {
    pub value: String,
}

#[derive(Debug, PartialEq)]
pub enum AWKPattern {
    BEGIN,
    END,
    Always,
}

#[derive(Debug, PartialEq)]
pub struct AWKAction {
    pub statement: AWKPrint,
}

#[derive(Debug, PartialEq)]
pub struct AWKPatternAction {
    pub pattern: AWKPattern,
    pub action: AWKAction,
}

#[derive(Debug, PartialEq, Clone)]
pub struct AWKNumber {
    pub int: i64,
    pub float: f64,
    pub is_float: bool,
}

impl AWKNumber {
    pub fn int(value: i64) -> AWKNumber {
        AWKNumber {
            int: value,
            float: 0.0,
            is_float: false,
        }
    }
    pub fn float(value: f64) -> AWKNumber {
        return if value == value as i64 as f64 {
            AWKNumber {
                int: value as i64,
                float: 0.0,
                is_float: false,
            }
        } else {
            AWKNumber {
                int: 0,
                float: value,
                is_float: true,
            }
        };
    }
}

#[derive(Debug, PartialEq)]
pub struct AWKPrint {
    // 一時的に
    pub exprlist: Vec<AWKNonUnaryPrintExpr>,
}
