/*
 * file: eval_expr.rs
 * author: kota kato 2022
 * description:
 *   Evaluate AWKExpr
 */

use crate::{ast::def::*, core::env::AWKEnv};

// AWKExpr
pub fn eval_awkexpr(expr: &AWKExpr, env: &mut AWKEnv) -> AWKVal {
    match expr {
        AWKExpr::Value(value) => value.clone(),
        AWKExpr::BinaryOperation { op, left, right } => eval_binary_operation(op, left, right, env),
        AWKExpr::Field(reference) => eval_fieldreference(reference, env),
        AWKExpr::Name(name) => eval_awkname(&name, env),
        AWKExpr::Assign { lval, expr } => eval_assign(lval, expr, env),
        AWKExpr::IncDec {
            is_post,
            is_inc,
            lval,
        } => eval_incdec(*is_post, *is_inc, lval, env),
        AWKExpr::UnaryOperation { expr, op } => eval_unary_operation(expr, op, env),
    }
}

// error handring
fn eval_binary_operation(
    op: &AWKBinaryOperation,
    left: &Box<AWKExpr>,
    right: &Box<AWKExpr>,
    env: &mut AWKEnv,
) -> AWKVal {
    let left = eval_awkexpr(left, env);
    let right = eval_awkexpr(right, env);
    return match op {
        AWKBinaryOperation::Add => left.add(&right),
        AWKBinaryOperation::Sub => left.sub(&right),
        AWKBinaryOperation::Mul => left.mul(&right),
        AWKBinaryOperation::Div => {
            if right.to_float() == 0.0 {
                println!("divisition by zero");
                todo!();
            };
            left.div(&right)
        }
        AWKBinaryOperation::Mod => left.module(&right),
        AWKBinaryOperation::Pow => left.pow(&right),
        AWKBinaryOperation::Cat => left.concat(&right),
        AWKBinaryOperation::And => left.and(&right),
        AWKBinaryOperation::Or => left.or(&right),
        AWKBinaryOperation::LessThan => left.lessthan(&right),
        AWKBinaryOperation::LessEqualThan => left.lessequalthan(&right),
        AWKBinaryOperation::NotEqual => left.notequal(&right),
        AWKBinaryOperation::Equal => left.equal(&right),
        AWKBinaryOperation::GreaterThan => left.greaterthan(&right),
        AWKBinaryOperation::GreaterEqualThan => left.greaterequalthan(&right),
    };
}

fn eval_fieldreference(reference: &Box<AWKExpr>, env: &mut AWKEnv) -> AWKVal {
    // -1などは整数に変換される
    // これは意図した動作ではない
    let n = eval_awkexpr(&reference, env).to_float() as usize;
    // TODO: handle Err
    env.get_field_n(n as usize).unwrap()
}

fn eval_awkname(name: &str, env: &mut AWKEnv) -> AWKVal {
    env.get_value(name)
}

fn eval_assign(lval: &AWKLval, expr: &Box<AWKExpr>, env: &mut AWKEnv) -> AWKVal {
    let val = eval_awkexpr(&expr, env);
    match lval {
        AWKLval::Name(name) => env.set_value(&name, &val),
        AWKLval::Field(e) => {
            let f = eval_awkexpr(e, env).to_float() as usize;
            env.set_field_n(f, &val);
        }
    };
    val
}

// TODO: REfactor
// AWKValに実装を移す必要がありそう
fn eval_incdec(is_post: bool, is_inc: bool, lval: &AWKLval, env: &mut AWKEnv) -> AWKVal {
    let addval = AWKVal::Num(if is_inc { 1.0 } else { -1.0 });
    if is_post {
        // 返す値を取得
        let ret = match lval {
            AWKLval::Name(name) => env.get_value(&name).to_float(),
            AWKLval::Field(e) => {
                let f = eval_awkexpr(e, env).to_float() as usize;
                // TODO: Error handling
                env.get_field_n(f).unwrap().to_float()
            }
        };
        // 加算or減算
        match lval {
            AWKLval::Name(name) => env.set_value(name, &env.get_value(name).add(&addval)),
            AWKLval::Field(expr) => {
                let f = eval_awkexpr(expr, env).to_float() as usize;
                env.set_field_n(f, &env.get_field_n(f).unwrap().add(&addval))
            }
        };
        AWKVal::Num(ret)
    } else {
        match lval {
            AWKLval::Name(name) => env.set_value(name, &env.get_value(name).add(&addval)),
            AWKLval::Field(expr) => {
                let f = eval_awkexpr(expr, env).to_float() as usize;
                env.set_field_n(f, &env.get_field_n(f).unwrap().add(&addval))
            }
        };
        let ret = match lval {
            AWKLval::Name(name) => env.get_value(&name).to_float(),
            AWKLval::Field(e) => {
                let f = eval_awkexpr(e, env).to_float() as usize;
                // TODO: Error handling
                env.get_field_n(f).unwrap().to_float()
            }
        };
        AWKVal::Num(ret)
    }
}

fn eval_unary_operation(expr: &Box<AWKExpr>, op: &AWKUnaryOperation, env: &mut AWKEnv) -> AWKVal {
    let val = eval_awkexpr(expr, env);
    match op {
        AWKUnaryOperation::Not => val.not(),
        AWKUnaryOperation::Plus => val.plus(),
        AWKUnaryOperation::Minus => val.minus(),
    }
}
