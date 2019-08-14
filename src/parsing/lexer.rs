use crate::parsing::reader::*;
use std::fmt::{ self, Display, Formatter };

#[derive(Debug, Clone)]
pub enum Token {
    Integer(String),
    Identifier(String),
    Keyword(String),
    Operator(String),
    OpenBrace,
    CloseBrace,
    OpenParenthesis,
    CloseParenthesis,
    Semicolon,
    Invalid(char),
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Token::Integer(data) => write!(f, "{}", data),
            Token::Identifier(data) => write!(f, "{}", data),
            Token::Keyword(data) => write!(f, "{}", data),
            Token::Operator(data) => write!(f, "{}", data),
            Token::OpenBrace => write!(f, "{{"),
            Token::CloseBrace => write!(f, "}}"),
            Token::OpenParenthesis => write!(f, "("),
            Token::CloseParenthesis => write!(f, ")"),
            Token::Semicolon => write!(f, ";"),
            Token::Invalid(c) => write!(f, "{}", c),
        }
    }
}

static KEYWORDS: &[&str] = &[
    "u8",
    "i8",
    "u16",
    "i16",
    "u32",
    "i32",
    "u64",
    "i64",
    "return",
    "=>",
    "->",
];

pub struct Lexer<'a> {
    reader: Reader<'a>,
}

impl<'a> Lexer<'a> {
    pub fn new(s: &'a str) -> Self {
        Lexer {
            reader: Reader::new(s),
        }
    }
}

pub fn is_operator(c: char) -> bool {
    "+-/%*><&|".contains(c)
}

pub fn is_integer(c: char) -> bool {
    c.is_digit(10)
}

pub fn is_alphanumeric(c: char) -> bool {
    c.is_ascii_alphanumeric()
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;
    fn next(&mut self) -> Option<Token> {
        self.reader
            .by_ref()
            .find(|c| !c.is_whitespace())
            .map(|c: char| match c {
                '(' => Token::OpenParenthesis,
                ')' => Token::CloseParenthesis,
                '{' => Token::OpenBrace,
                '}' => Token::CloseBrace,
                ';' => Token::Semicolon,
                '-' | '=' if self.reader.peek() == Some('>') => {
                    self.reader.next();
                    Token::Keyword(format!("{}{}", c, '>'))
                },
                c if is_operator(c) => Token::Operator(c.to_string()),
                c if is_integer(c) => {
                    let mut s: String = self.reader.read_while(is_integer);
                    s.insert(0, c);
                    Token::Integer(s)
                },
                c if is_alphanumeric(c) => {
                    let mut s: String = self.reader.read_while(is_alphanumeric);
                    s.insert(0, c);

                    if KEYWORDS.contains(&s.as_str()) {
                        Token::Keyword(s)
                    } else {
                        Token::Identifier(s)
                    }
                },
                c => Token::Invalid(c),
            })
    }
}
