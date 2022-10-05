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
    Expr(AWKExpr),
}

#[derive(Debug, PartialEq)]
pub enum AWKStat {
    Expr(AWKExpr),
    Print(AWKPrint),
    Action(Vec<AWKStat>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum AWKExpr {
    // 値
    Value(AWKVal),
    // 名前
    Name(String),
    // 二項演算子
    BinaryOperation {
        op: AWKBinaryOperation,
        left: Box<AWKExpr>,
        right: Box<AWKExpr>,
    },
    // フィールドリファレンス
    Field(Box<AWKExpr>),
    // 代入
    Assign {
        lval: AWKLval,
        expr: Box<AWKExpr>,
    },
    // インクリメント・ディクリメント
    IncDec {
        is_post: bool,
        is_inc: bool,
        lval: AWKLval,
    },
    // 単項演算子
    UnaryOperation {
        op: AWKUnaryOperation,
        expr: Box<AWKExpr>,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub enum AWKLval {
    Name(String),
    Field(Box<AWKExpr>),
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum AWKBinaryOperation {
    Add,              // +
    Sub,              // -
    Mul,              // *
    Div,              // /
    Pow,              // ^
    Mod,              // %
    Cat,              // string concat
    And,              // &&
    Or,               // ||
    LessThan,         // <
    LessEqualThan,    // <=
    NotEqual,         // !=
    Equal,            // ==
    GreaterThan,      // >
    GreaterEqualThan, // >=
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum AWKUnaryOperation {
    Not,   // !
    Plus,  // +
    Minus, // -
}

#[derive(Debug, PartialEq, Clone)]
pub enum AWKVal {
    Num(AWKFloat),
    Str(AWKStr),
    None,
}

pub type AWKStr = String;
pub type AWKFloat = f64;

#[derive(Debug, PartialEq)]
pub struct AWKPrint {
    // 一時的に
    pub exprlist: Vec<AWKExpr>,
}
