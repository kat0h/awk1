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
        AWKExpr::FieldReference(reference) => eval_fieldreference(reference, env),
        AWKExpr::Name(name) => eval_awkname(&name, env),
    }
}

// error handring
fn eval_binary_operation(
    op: &AWKOperation,
    left: &Box<AWKExpr>,
    right: &Box<AWKExpr>,
    env: &mut AWKEnv,
) -> AWKVal {
    let left = eval_awkexpr(left, env).to_float();
    let right = eval_awkexpr(right, env).to_float();
    return AWKVal::Num(match op {
        AWKOperation::Add => left + right,
        AWKOperation::Sub => left - right,
        AWKOperation::Mul => left * right,
        AWKOperation::Div => {
            if right == 0.0 {
                println!("divisition by zero");
                todo!();
            }
            left / right
        }
    });
}

fn eval_fieldreference(reference: &Box<AWKExpr>, env: &mut AWKEnv) -> AWKVal {
    let n = eval_awkexpr(&reference, env).to_float() as usize;
    AWKVal::Str(env.get_field(n as usize).unwrap())
}

fn eval_awkname(name: &str, env: &mut AWKEnv) -> AWKVal {
    env.get_value(name)
}
