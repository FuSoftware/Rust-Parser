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
    EOF,
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
            _ => "Undefined",
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

    pub fn read_while(&mut self, f: impl Fn(char) -> bool) -> String {
        let mut s: String = String::new();

        while !self.reader.eof() && f(self.reader.peek()) {
            s.push(self.reader.next());
        }

        s
    }

    pub fn next(&mut self) -> Token {
        if self.reader.eof() {
            Token::from_type(TokenType::EOF)
        } else {
            let c: char = self.reader.next();
            let mut tt: TokenType = self.scan(c);
            match tt {
                TokenType::OpenParenthesis
                | TokenType::CloseParenthesis
                | TokenType::OpenBrace
                | TokenType::CloseBrace
                | TokenType::Semicolon => Token::from_char(tt, c),
                TokenType::Operator => {
                    if ((c == '-') | (c == '=')) && (self.reader.peek() == '>') {
                        self.reader.next();
                        let mut s: String = String::new();
                        s.push(c);
                        s.push('>');
                        Token::new(TokenType::Keyword, s)
                    } else {
                        Token::from_char(tt, c)
                    }
                },
                TokenType::Identifier => {
                    let mut s: String = self.read_while(Lexer::is_alphanumeric);
                    s.insert(0, c);

                    if KEYWORDS.contains(&s.as_str()) {
                        tt = TokenType::Keyword;
                    }

                    Token::new(tt, s)
                },
                TokenType::Integer => {
                    let mut s: String = self.read_while(Lexer::is_integer);
                    s.insert(0, c);
                    Token::new(tt, s)
                },
                TokenType::Whitespace => {
                    self.read_while(Lexer::is_whitespace);
                    Token::new(tt, String::new())
                },
                _ => Token::new(tt, String::from("")),
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

    pub fn scan(&self, c: char) -> TokenType {
        if c == '{' {
            TokenType::OpenBrace
        } else if c == '}' {
            TokenType::CloseBrace
        } else if c == '(' {
            TokenType::OpenParenthesis
        } else if c == ')' {
            TokenType::CloseParenthesis
        } else if c == ';' {
            TokenType::Semicolon
        } else if Lexer::is_operator(c) {
            TokenType::Operator
        } else if Lexer::is_integer(c) {
            TokenType::Integer
        } else if Lexer::is_alphanumeric(c) {
            TokenType::Identifier
        } else if Lexer::is_whitespace(c) {
            TokenType::Whitespace
        } else {
            TokenType::Invalid
        }
    }

    pub fn eof(&self) -> bool {
        self.reader.eof()
    }
}
