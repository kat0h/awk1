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
    AWKPrint(AWKPrint),
}

#[derive(Debug, PartialEq)]
pub enum AWKExpr {
    AWKValue(AWKValue),
    AWKBinaryOperation {
        op: AWKOperation,
        left: Box<AWKExpr>,
        right: Box<AWKExpr>,
    },
}

#[derive(Debug, PartialEq)]
pub enum AWKOperation {
    Plus,
    Minus
}

#[derive(Debug, PartialEq, Clone)]
pub enum AWKValue {
    AWKNumber(AWKNumber),
    AWKString(AWKString),
}

#[derive(Debug, PartialEq, Clone)]
pub struct AWKString {
    pub value: String,
}

#[derive(Debug, PartialEq, Clone)]
pub enum AWKNumber {
    Int(i64),
    Float(f64),
}

#[derive(Debug, PartialEq)]
pub struct AWKPrint {
    // 一時的に
    pub exprlist: Vec<Box<AWKExpr>>,
}
