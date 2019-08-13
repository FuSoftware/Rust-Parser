mod lib;
use std::fs;
use lib::parsing::lexer::*;

fn main() {
    let s: String = fs::read_to_string("sample/main.rs").unwrap();
    println!("{}", s);
    let mut l: Lexer = Lexer::new(s);
    let mut v: Vec<Token> = Vec::new();

    loop {
        let t: Token = l.next();
        v.push(t.clone());

        let tt: TokenType = t.token_type;
        let td: String = t.data;
        
        match tt {
            TokenType::EOF => {
                break;
            }
            TokenType::Whitespace => {

            },
            _ => {
                println!("{:20} : {}", tt.label(), td);
            }
        } 
    }
}
