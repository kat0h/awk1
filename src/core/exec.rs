/*
 * file: exec.rs
 * author: kota kato 2022
 * description:
 *   Execute ast
 */

use crate::ast::def::*;
use crate::core::AWKEnv;
use std::io;

pub fn read_line_and_exec_program(program: &AWKProgram, env: &mut AWKEnv) {
    // TODO: IF AWKProgram has BEGIN or END pattern only, Skip main loop
    loop {
        // Read one line from stdin
        let mut line = String::new();
        if io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line.")
            != 0
        {
            env.set_field(&line);
            env.set_value("NR", &AWKVal::Num(to_awknum(env.get_value("NR")) + 1.0));

            for i in &program.item_list {
                match i {
                    AWKItem::PatternAction(pattern_action) => {
                        match pattern_action.pattern {
                            AWKPattern::Always => exec_awkaction(&pattern_action.action, env),
                            _ => todo!(),
                        };
                    }
                };
            }
        } else {
            break;
        }
    }
}

pub fn exec_all_begin_pattern(program: &AWKProgram, env: &mut AWKEnv) {
    // find BEGIN pattern
    for i in &program.item_list {
        match i {
            AWKItem::PatternAction(pattern_action) => {
                match pattern_action.pattern {
                    AWKPattern::Begin => {
                        exec_awkaction(&pattern_action.action, env);
                    }
                    _ => (),
                };
            }
        };
    }
}

pub fn exec_all_end_pattern(program: &AWKProgram, env: &mut AWKEnv) {
    // find BEGIN pattern
    for i in &program.item_list {
        match i {
            AWKItem::PatternAction(pattern_action) => {
                match pattern_action.pattern {
                    AWKPattern::End => {
                        exec_awkaction(&pattern_action.action, env);
                    }
                    _ => (),
                };
            }
        };
    }
}

// AWKPatternAction
pub fn exec_awkaction(actions: &Vec<AWKStat>, env: &mut AWKEnv) {
    for statement in actions {
        match statement {
            AWKStat::Print(awkprint) => exec_awkprint(&awkprint, env),
        };
    }
}

// print statement
pub fn exec_awkprint(awkprint: &AWKPrint, env: &mut AWKEnv) {
    let mut s = false;
    for expr in &awkprint.exprlist {
        print!(
            "{}{}",
            if s { " " } else { "" },
            to_awkstr(eval_awkexpr(expr, env))
        );
        s = true;
    }
    println!();
}

// AWKExpr
pub fn eval_awkexpr(expr: &AWKExpr, env: &mut AWKEnv) -> AWKVal {
    match expr {
        AWKExpr::Value(value) => value.clone(),
        AWKExpr::BinaryOperation { op, left, right } => eval_binary_operation(op, left, right, env),
        AWKExpr::FieldReference(reference) => eval_fieldreference(reference, env),
        AWKExpr::Name(name) => eval_awkname(&name, env)
    }
}

// error handring
pub fn eval_binary_operation(
    op: &AWKOperation,
    left: &Box<AWKExpr>,
    right: &Box<AWKExpr>,
    env: &mut AWKEnv,
) -> AWKVal {
    let left = to_awknum(eval_awkexpr(left, env));
    let right = to_awknum(eval_awkexpr(right, env));
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

pub fn eval_fieldreference(reference: &Box<AWKExpr>, env: &mut AWKEnv) -> AWKVal {
    let n = match eval_awkexpr(&reference, env) {
        AWKVal::Num(n) => n as usize,
        AWKVal::Str(_) => todo!(),
    };
    AWKVal::Str(env.get_field(n as usize).unwrap())
}

pub fn eval_awkname(name: &str, env: &mut AWKEnv) -> AWKVal {
    env.get_value(name)
}

// AWKValue -> AWKNum / AWKStr
pub fn to_awkstr(value: AWKVal) -> AWKStr {
    match value {
        AWKVal::Num(n) => n.to_string(),
        AWKVal::Str(s) => s.clone(),
    }
}

pub fn to_awknum(value: AWKVal) -> AWKFloat {
    use crate::ast::number::parse_number;
    match value {
        AWKVal::Num(n) => n,
        AWKVal::Str(s) => match parse_number(&s) {
            Ok((_, n)) => n,
            Err(_) => 0.0,
        },
    }
}
