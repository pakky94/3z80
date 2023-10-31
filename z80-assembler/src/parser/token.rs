#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Token {
    Label(String),
    Identifier(String),
    OpenParen,
    CloseParen,
    Plus,
    Value(u16),
    Comma,
    NewLine,
    EOF,
}
