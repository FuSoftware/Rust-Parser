mod lib;
use lib::parsing::lexer::*;
use std::fs;

fn main() {
    let s: String = fs::read_to_string("sample/main.rs").unwrap();
    println!("{}", s);
    let l: Lexer = Lexer::new(&s);
    let v: Vec<Token> = l.inspect(|t| println!("{:20} : {}", t.token_type.label(), t.data))
        .collect();
}
