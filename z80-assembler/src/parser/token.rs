#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Token {
    Label(String),
    Identifier(String),
    OpenParen,
    CloseParen,
    Plus,
    ShortValue(u8),
    WideValue(u16),
    Comma,
    NewLine,
    EOF,
}
