#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum Token {
    Identifier(String),
    Constant(Constant),
    Symbol(Symbol),
    Keyword(Keyword),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum Symbol {
    LeftBrace,
    RightBrace,
    LeftParen,
    RightParen,
    Plus,
    Minus,
    Star,
    Slash,
    Modulo,
    Equal,
    NotEqual,
    Less,
    LessEqual,
    More,
    MoreEqual,
    Or,
    And,
    Not,
    Assign,
    Comma,
    Semicolon,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum Keyword {
    Int,
    Void,
    If,
    Else,
    While,
    Return,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum Constant {
    Int(i32),
}

impl Constant {
    pub(crate) fn new(input: &str) -> Result<Self, ()> {
        if let Ok(value) = input.parse::<i32>() {
            return Ok(Self::Int(value));
        }
        Err(())
    }
}