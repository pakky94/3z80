#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Token {
    Identifier(String),
    OpenParen,
    CloseParen,
    Plus,
    Value(u16),
    Comma,
    Dot,
    Colon,
    NewLine,
    EOF,
}
