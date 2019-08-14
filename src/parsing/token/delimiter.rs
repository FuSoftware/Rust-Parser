use std::fmt::{ self, Display, Formatter };

#[derive(Debug, Clone)]
pub struct Delimiter {
    bracket: Bracket,
    orientation: Orientation,
}

impl Delimiter {
    pub const fn new(bracket: Bracket, orientation: Orientation) -> Self {
        Delimiter { bracket, orientation }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Bracket {
    Curly,
    Square,
    Parentheses,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Orientation {
    Open,
    Close,
}

impl Display for Delimiter {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match (&self.bracket, &self.orientation) {
            (Bracket::Curly, Orientation::Open) => write!(f, "{{"),
            (Bracket::Curly, Orientation::Close) => write!(f, "}}"),
            (Bracket::Square, Orientation::Open) => write!(f, "["),
            (Bracket::Square, Orientation::Close) => write!(f, "]"),
            (Bracket::Parentheses, Orientation::Open) => write!(f, "("),
            (Bracket::Parentheses, Orientation::Close) => write!(f, ")"),
        }
    }
}