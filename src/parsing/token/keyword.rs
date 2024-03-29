use std::fmt::{ self, Display, Formatter };

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Keyword {
    As,
    Break,
    Const,
    Continue,
    Crate,
    Else,
    Enum,
    Extern,
    False,
    Fn,
    For,
    If,
    Impl,
    In,
    Let,
    Loop,
    Match,
    Mod,
    Move,
    Mut,
    Pub,
    Ref,
    Return,
    SelfValue,
    SelfType,
    Static,
    Struct,
    Super,
    Trait,
    True,
    Type,
    Unsafe,
    Use,
    Where,
    While,
    Dyn,
    Union,
    StaticLifetime,
    Abstract,
    Become,
    Box,
    Do,
    Final,
    Macro,
    Override,
    Priv,
    Typeof,
    Unsized,
    Virtual,
    Yield,
    Async,
    Await,
    Try,
}

impl Keyword {
    pub fn from_ident(ident: String) -> Result<Keyword, String> {
        match ident.as_str() {
            "as" => Ok(Keyword::As),
            "break" => Ok(Keyword::Break),
            "const" => Ok(Keyword::Const),
            "continue" => Ok(Keyword::Continue),
            "crate" => Ok(Keyword::Crate),
            "else" => Ok(Keyword::Else),
            "enum" => Ok(Keyword::Enum),
            "extern" => Ok(Keyword::Extern),
            "false" => Ok(Keyword::False),
            "fn" => Ok(Keyword::Fn),
            "for" => Ok(Keyword::For),
            "if" => Ok(Keyword::If),
            "impl" => Ok(Keyword::Impl),
            "in" => Ok(Keyword::In),
            "let" => Ok(Keyword::Let),
            "loop" => Ok(Keyword::Loop),
            "match" => Ok(Keyword::Match),
            "mod" => Ok(Keyword::Mod),
            "move" => Ok(Keyword::Move),
            "mut" => Ok(Keyword::Mut),
            "pub" => Ok(Keyword::Pub),
            "ref" => Ok(Keyword::Ref),
            "return" => Ok(Keyword::Return),
            "selfvalue" => Ok(Keyword::SelfValue),
            "selftype" => Ok(Keyword::SelfType),
            "static" => Ok(Keyword::Static),
            "struct" => Ok(Keyword::Struct),
            "super" => Ok(Keyword::Super),
            "trait" => Ok(Keyword::Trait),
            "true" => Ok(Keyword::True),
            "type" => Ok(Keyword::Type),
            "unsafe" => Ok(Keyword::Unsafe),
            "use" => Ok(Keyword::Use),
            "where" => Ok(Keyword::Where),
            "while" => Ok(Keyword::While),
            "dyn" => Ok(Keyword::Dyn),
            "union" => Ok(Keyword::Union),
            "staticlifetime" => Ok(Keyword::StaticLifetime),
            "abstract" => Ok(Keyword::Abstract),
            "become" => Ok(Keyword::Become),
            "box" => Ok(Keyword::Box),
            "do" => Ok(Keyword::Do),
            "final" => Ok(Keyword::Final),
            "macro" => Ok(Keyword::Macro),
            "override" => Ok(Keyword::Override),
            "priv" => Ok(Keyword::Priv),
            "typeof" => Ok(Keyword::Typeof),
            "unsized" => Ok(Keyword::Unsized),
            "virtual" => Ok(Keyword::Virtual),
            "yield" => Ok(Keyword::Yield),
            "async" => Ok(Keyword::Async),
            "await" => Ok(Keyword::Await),
            "try" => Ok(Keyword::Try),
            _ => Err(ident),
        }
    }
}

impl Display for Keyword {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Keyword::As => write!(f, "as"),
            Keyword::Break => write!(f, "break"),
            Keyword::Const => write!(f, "const"),
            Keyword::Continue => write!(f, "continue"),
            Keyword::Crate => write!(f, "crate"),
            Keyword::Else => write!(f, "else"),
            Keyword::Enum => write!(f, "enum"),
            Keyword::Extern => write!(f, "extern"),
            Keyword::False => write!(f, "false"),
            Keyword::Fn => write!(f, "fn"),
            Keyword::For => write!(f, "for"),
            Keyword::If => write!(f, "if"),
            Keyword::Impl => write!(f, "impl"),
            Keyword::In => write!(f, "in"),
            Keyword::Let => write!(f, "let"),
            Keyword::Loop => write!(f, "loop"),
            Keyword::Match => write!(f, "match"),
            Keyword::Mod => write!(f, "mod"),
            Keyword::Move => write!(f, "move"),
            Keyword::Mut => write!(f, "mut"),
            Keyword::Pub => write!(f, "pub"),
            Keyword::Ref => write!(f, "ref"),
            Keyword::Return => write!(f, "return"),
            Keyword::SelfValue => write!(f, "selfvalue"),
            Keyword::SelfType => write!(f, "selftype"),
            Keyword::Static => write!(f, "static"),
            Keyword::Struct => write!(f, "struct"),
            Keyword::Super => write!(f, "super"),
            Keyword::Trait => write!(f, "trait"),
            Keyword::True => write!(f, "true"),
            Keyword::Type => write!(f, "type"),
            Keyword::Unsafe => write!(f, "unsafe"),
            Keyword::Use => write!(f, "use"),
            Keyword::Where => write!(f, "where"),
            Keyword::While => write!(f, "while"),
            Keyword::Dyn => write!(f, "dyn"),
            Keyword::Union => write!(f, "union"),
            Keyword::StaticLifetime => write!(f, "staticlifetime"),
            Keyword::Abstract => write!(f, "abstract"),
            Keyword::Become => write!(f, "become"),
            Keyword::Box => write!(f, "box"),
            Keyword::Do => write!(f, "do"),
            Keyword::Final => write!(f, "final"),
            Keyword::Macro => write!(f, "macro"),
            Keyword::Override => write!(f, "override"),
            Keyword::Priv => write!(f, "priv"),
            Keyword::Typeof => write!(f, "typeof"),
            Keyword::Unsized => write!(f, "unsized"),
            Keyword::Virtual => write!(f, "virtual"),
            Keyword::Yield => write!(f, "yield"),
            Keyword::Async => write!(f, "async"),
            Keyword::Await => write!(f, "await"),
            Keyword::Try => write!(f, "try"),
        }
    }
}