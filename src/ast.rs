/*
 * file: ast.rs
 * author: kota kato 2020
 * description:
 *   make ast/ public
 */

pub mod def;
pub mod expr;
pub mod number;
pub mod patternaction;
pub mod statement;
pub mod string;
pub mod print;

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
