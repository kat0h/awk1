/*
 * file: exec.rs
 * author: kota kato 2022
 * description:
 *   Execute ast
 */

use crate::{
    ast::def::*,
    core::{eval_expr::eval_awkexpr, AWKEnv},
};
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
            env.set_value("NR", &env.get_value("NR").add(&AWKVal::Num(1.0)));

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
            AWKStat::Expr(expr) => {
                eval_awkexpr(expr, env);
            }
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
            eval_awkexpr(expr, env).to_str()
        );
        s = true;
    }
    println!();
}
