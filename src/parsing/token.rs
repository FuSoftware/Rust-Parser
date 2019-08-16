use std::fmt::{ self, Display, Formatter };

pub mod keyword;
pub mod punctuation;
pub mod delimiter;

pub use self::{
    keyword::Keyword,
    punctuation::Punctuation,
    delimiter::*,
};

#[derive(Debug, Clone)]
pub enum Token {
    Integer(String),
    Identifier(String),
    Keyword(Keyword),
    Punctuation(Punctuation),
    Delimiter(Delimiter),
    Invalid(char),
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Token::Integer(num) => num.fmt(f),
            Token::Identifier(ident) => ident.fmt(f),
            Token::Keyword(key) => key.fmt(f),
            Token::Punctuation(punct) => punct.fmt(f),
            Token::Delimiter(del) => del.fmt(f),
            Token::Invalid(c) => c.fmt(f),
        }
    }
}

impl From<Keyword> for Token {
    fn from(keyword: Keyword) -> Self {
        Token::Keyword(keyword)
    }
}

impl From<Punctuation> for Token {
    fn from(punctuation: Punctuation) -> Self {
        Token::Punctuation(punctuation)
    }
}

impl From<Delimiter> for Token {
    fn from(delimiter: Delimiter) -> Self {
        Token::Delimiter(delimiter)
    }
}
