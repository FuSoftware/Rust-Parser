use rust_compiler::parsing::lexer::Lexer;
use rust_compiler::parsing::token::Token;
use std::fs;

fn main() {
    let s: String = fs::read_to_string("sample/main.rs").unwrap();
    println!("{}", s);
    let l: Lexer = Lexer::new(&s);
    let v: Vec<Token> = l.inspect(|t| println!("{:?}", t)).collect();
    
    for t in v {
        print!("{} ", t)
    }
    println!()
}
