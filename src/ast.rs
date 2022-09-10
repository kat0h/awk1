/*
 * file: ast.rs
 * author: kota kato 2022
 * description:
 *   make ast/ public
 */

pub mod def;
pub mod number;
pub mod patternaction;
pub mod print;
pub mod print_expr;
pub mod statement;
pub mod string;

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
