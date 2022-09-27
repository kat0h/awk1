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
        AWKExpr::Assign { lval, expr } => eval_assign(lval, expr, env),
    }
}

// error handring
fn eval_binary_operation(
    op: &AWKOperation,
    left: &Box<AWKExpr>,
    right: &Box<AWKExpr>,
    env: &mut AWKEnv,
) -> AWKVal {
    let left = eval_awkexpr(left, env);
    let right = eval_awkexpr(right, env);
    return AWKVal::Num(match op {
        AWKOperation::Add => left.add(&right).to_float(),
        AWKOperation::Sub => left.sub(&right).to_float(),
        AWKOperation::Mul => left.mul(&right).to_float(),
        AWKOperation::Div => {
            if right.to_float() == 0.0 {
                println!("divisition by zero");
                todo!();
            };
            left.div(&right).to_float()
        }
        AWKOperation::Mod => left.module(&right).to_float(),
    });
}

fn eval_fieldreference(reference: &Box<AWKExpr>, env: &mut AWKEnv) -> AWKVal {
    let n = eval_awkexpr(&reference, env).to_float() as usize;
    AWKVal::Str(env.get_field(n as usize).unwrap())
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
