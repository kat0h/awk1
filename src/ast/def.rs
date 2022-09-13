/*
 * file: def.rs
 * author: kota kato 2022
 * description:
 *   definition ast nodes
 *   https://pubs.opengroup.org/onlinepubs/9699919799/utilities/awk.html
 */

// AWKProgram is item_list separatedd by terminator (; or \n)
#[derive(Debug, PartialEq)]
pub struct AWKProgram {
    pub item_list: Vec<AWKItem>,
}

#[derive(Debug, PartialEq)]
pub enum AWKItem {
    AWKPatternAction(AWKPatternAction),
}

#[derive(Debug, PartialEq)]
pub struct AWKPatternAction {
    pub pattern: AWKPattern,
    pub action: Vec<AWKStatement>,
}

#[derive(Debug, PartialEq)]
pub enum AWKPattern {
    Always,
    Begin,
    End,
}

#[derive(Debug, PartialEq)]
pub enum AWKStatement {
    AWKPrint(AWKPrint)
}

#[derive(Debug, PartialEq)]
pub enum AWKExpr {
    AWKNumber(AWKNumber),
    AWKString(AWKString),
}

#[derive(Debug, PartialEq, Clone)]
pub struct AWKString {
    pub value: String,
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
    pub exprlist: Vec<AWKExpr>,
}
