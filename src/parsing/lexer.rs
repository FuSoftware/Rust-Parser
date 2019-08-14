use super::reader::Reader;
use super::token::*;

pub struct Lexer<'a> {
    reader: Reader<'a>,
}

impl<'a> Lexer<'a> {
    pub fn new(s: &'a str) -> Self {
        Lexer {
            reader: Reader::new(s),
        }
    }

    fn peek_one<T>(&mut self, c: char, v: T) -> Option<T> {
        if self.reader.peek() == Some(c) {
            self.reader.next();
            Some(v)
        } else {
            None
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

macro_rules! peek {
    ($e:ident; $($c:literal => $v:expr,)+ _ => $d:expr $(,)?) => {
        match $e.reader.peek() {
            $(Some($c) => { $e.reader.next(); $v },)+
            _ => $d,
        }
    };
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;
    fn next(&mut self) -> Option<Token> {
        self.reader
            .by_ref()
            .find(|c| !c.is_whitespace())
            .map(|c: char| match c {
                '{' => Delimiter::new(Bracket::Curly, Orientation::Open).into(),
                '}' => Delimiter::new(Bracket::Curly, Orientation::Close).into(),
                '[' => Delimiter::new(Bracket::Square, Orientation::Open).into(),
                ']' => Delimiter::new(Bracket::Square, Orientation::Close).into(),
                '(' => Delimiter::new(Bracket::Parentheses, Orientation::Open).into(),
                ')' => Delimiter::new(Bracket::Parentheses, Orientation::Close).into(),
                ';' => Punctuation::Semi.into(),
                '+' => peek! { self;
                    '=' => Punctuation::PlusEq.into(),
                    _ => Punctuation::Plus.into(),
                },
                '-' => peek! { self;
                    '=' => Punctuation::MinusEq.into(),
                    '>' => Punctuation::RArrow.into(),
                    _ => Punctuation::Minus.into(),
                },
                '*' => peek! { self;
                    '=' => Punctuation::StarEq.into(),
                    _ => Punctuation::Star.into(),
                },
                '/' => peek! { self;
                    '=' => Punctuation::SlashEq.into(),
                    _ => Punctuation::Slash.into(),
                },
                '%' => peek! { self;
                    '=' => Punctuation::PercentEq.into(),
                    _ => Punctuation::Percent.into(),
                },
                '^' => peek! { self;
                    '=' => Punctuation::CaretEq.into(),
                    _ => Punctuation::Caret.into(),
                },
                '!' => peek! { self;
                    '=' => Punctuation::Ne.into(),
                    _ => Punctuation::Not.into(),
                },
                '&' => peek! { self;
                    '=' => Punctuation::AndEq.into(),
                    '&' => Punctuation::AndAnd.into(),
                    _ => Punctuation::And.into(),
                },
                '|' => peek! { self;
                    '=' => Punctuation::OrEq.into(),
                    '|' => Punctuation::OrOr.into(),
                    _ => Punctuation::Or.into(),
                },
                '<' => peek! { self;
                    '<' => peek! { self;
                        '=' => Punctuation::ShlEq.into(),
                        _ => Punctuation::Shl.into(),
                    },
                    '=' => Punctuation::Le.into(),
                    _ => Punctuation::Lt.into(),
                },
                '>' => peek! { self;
                    '>' => peek! { self;
                        '=' => Punctuation::ShrEq.into(),
                        _ => Punctuation::Shr.into(),
                    },
                    '=' => Punctuation::Ge.into(),
                    _ => Punctuation::Gt.into(),
                },
                '=' => peek! { self;
                    '=' => Punctuation::EqEq.into(),
                    '>' => Punctuation::FatArrow.into(),
                    _ => Punctuation::Eq.into(),
                },
                '@' => Punctuation::At.into(),
                '_' => Punctuation::Underscore.into(),
                '.' => peek! { self;
                    '.' => peek! { self;
                        '=' => Punctuation::DotDotEq.into(),
                        '.' => Punctuation::DotDotDot.into(),
                        _ => Punctuation::DotDot.into(),
                    },
                    _ => Punctuation::Dot.into(),
                },
                c if is_integer(c) => {
                    let mut s: String = self.reader.read_while(is_integer);
                    s.insert(0, c);
                    Token::Integer(s)
                },
                c if is_alphanumeric(c) => {
                    let mut s: String = self.reader.read_while(is_alphanumeric);
                    s.insert(0, c);

                    Keyword::from_ident(s).map(Token::Keyword).unwrap_or_else(Token::Identifier)
                },
                c => Token::Invalid(c),
            })
    }
}
