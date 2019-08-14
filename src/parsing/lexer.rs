use crate::parsing::reader::*;

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub data: String,
}

impl Token {
    pub fn new(token_type: TokenType, data: String) -> Token {
        Token { token_type, data }
        }

    pub fn from_char(token_type: TokenType, c: char) -> Token {
        Token::new(token_type, c.to_string())
    }

    pub fn from_type(token_type: TokenType) -> Token {
        Token::new(token_type, String::new())
    }
}

#[derive(Debug, Copy, Clone)]
pub enum TokenType {
    OpenBrace,
    CloseBrace,
    OpenParenthesis,
    CloseParenthesis,
    Identifier,
    Semicolon,
    Keyword,
    Integer,
    Invalid,
    Whitespace,
    ReturnType,
    Operator,
}

impl TokenType {
    pub fn label(&self) -> &str {
        match self {
            TokenType::OpenBrace => "Open Brace",
            TokenType::CloseBrace => "Close Brace",
            TokenType::OpenParenthesis => "Open Parenthesis",
            TokenType::CloseParenthesis => "Close Parenthesis",
            TokenType::Identifier => "Identifier",
            TokenType::Semicolon => "Semicolon",
            TokenType::Keyword => "Keyword",
            TokenType::Integer => "Integer",
            TokenType::Invalid => "Invalid",
            TokenType::Whitespace => "Whitespace",
            TokenType::ReturnType => "Return Type",
            TokenType::Operator => "Operator",
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
    pub fn new (s: &'a str) -> Self {
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

pub fn is_whitespace(c: char) -> bool {
    c.is_whitespace()
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;
    fn next(&mut self) -> Option<Token> {
        self.reader.next().map(|c: char| match c {
            '(' => Token::from_char(TokenType::OpenParenthesis, c),
            ')' => Token::from_char(TokenType::CloseParenthesis, c),
            '{' => Token::from_char(TokenType::OpenBrace, c),
            '}' => Token::from_char(TokenType::CloseBrace, c),
            ';' => Token::from_char(TokenType::Semicolon, c),
            '-' | '=' if self.reader.peek() == Some('>') => {
                self.reader.next();
                Token::new(TokenType::Keyword, format!("{}{}", c, '>'))
            },
            c if is_operator(c) => Token::from_char(TokenType::Operator, c),
            c if is_integer(c) => {
                let mut s: String = self.reader.read_while(is_integer);
                s.insert(0, c);
                Token::new(TokenType::Integer, s)
            },
            c if is_alphanumeric(c) => {
                let mut s: String = self.reader.read_while(is_alphanumeric);
                s.insert(0, c);

                let tt = if KEYWORDS.contains(&s.as_str()) {
                    TokenType::Keyword
                } else {
                    TokenType::Identifier
                };

                Token::new(tt, s)
            },
            c if is_whitespace(c) => {
                self.reader.read_while(is_whitespace);
                Token::from_type(TokenType::Whitespace)
            },
            c => Token::from_char(TokenType::Invalid, c),
        })
    }
}
