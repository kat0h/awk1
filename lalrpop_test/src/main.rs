
fn main() {
    println!("Hello, world!");
}

#[test]
fn calc() {
    use lalrpop_util::lalrpop_mod;
    lalrpop_mod!(grammer);
    let v = grammer::ExprParser::new().parse("1+2").unwrap();
    dbg!(v);
}
