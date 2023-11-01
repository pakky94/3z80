#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Token {
    Identifier(String),
    OpenParen,
    CloseParen,
    Plus,
    Value(u16, u8),
    Comma,
    Dot,
    Colon,
    Amp,
    Asterisk,
    NewLine,
    EOF,
}
