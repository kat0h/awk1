/* 
 * file: ast.rs
 * author: kota kato 2020
 * description:
 *   make ast/ public
 */

pub mod number;
pub mod string;
pub mod patternaction;
pub mod expr;
pub mod statement;
pub mod def;


/* 
 * Expr : AWKNUMBER | AWKSTRING
 */

/*
 *  paction : pattern '{' action '}'
 *  pattern : BEGIN
 *          | END
 *          |
 *  action  : print '(' expr ')'
 *          | print expr
*/
