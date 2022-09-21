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
    PatternAction(AWKPatternAction),
}

#[derive(Debug, PartialEq)]
pub struct AWKPatternAction {
    pub pattern: AWKPattern,
    pub action: Vec<AWKStat>, // action is vector of statement
}

#[derive(Debug, PartialEq)]
pub enum AWKPattern {
    Always,
    Begin,
    End,
}

#[derive(Debug, PartialEq)]
pub enum AWKStat {
    Print(AWKPrint),
}

#[derive(Debug, PartialEq)]
pub enum AWKExpr {
    Value(AWKVal),
    BinaryOperation {
        op: AWKOperation,
        left: Box<AWKExpr>,
        right: Box<AWKExpr>,
    },
}

#[derive(Debug, PartialEq)]
pub enum AWKOperation {
    Add, // +
    Sub, // -
    Mul, // *
    Div, // /
}

#[derive(Debug, PartialEq, Clone)]
pub enum AWKVal {
    Num(AWKNum),
    Str(AWKStr),
}

pub type AWKStr = String;

#[derive(Debug, PartialEq, Clone)]
pub enum AWKNum {
    Int(i64),
    Float(f64),
}

#[derive(Debug, PartialEq)]
pub struct AWKPrint {
    // 一時的に
    pub exprlist: Vec<Box<AWKExpr>>,
}
