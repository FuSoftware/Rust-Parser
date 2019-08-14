mod lib;
use lib::parsing::lexer::*;
use std::fs;

fn main() {
    let s: String = fs::read_to_string("sample/main.rs").unwrap();
    println!("{}", s);
    let l: Lexer = Lexer::new(&s);
    let v: Vec<Token> = l.inspect(|t| {
        match t.token_type {
            TokenType::Whitespace => (),
            tt => println!("{:20} : {}", tt.label(), t.data),
        } 
    }).collect();
}
