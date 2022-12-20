use lalrpop_util::lalrpop_mod;
lalrpop_mod!(grammer);

mod ast;

fn main() {
    let v = grammer::ParserParser::new().parse("2+2-(2*2)");
    dbg!(v);
}
