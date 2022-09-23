/*
 * file: core.rs
 * author: kota kato 2022
 * description:
 *   AST Walker Core
 */

mod env;
mod eval_expr;
mod exec;
mod util;
mod awkval;

use crate::ast::def::AWKProgram;
use env::AWKEnv;
use exec::*;

pub fn exec_program(program: &AWKProgram) {
    let mut env = AWKEnv::new();

    exec_all_begin_pattern(&program, &mut env);
    read_line_and_exec_program(&program, &mut env);
    exec_all_end_pattern(&program, &mut env);
}
