#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Token {
    pub token: TokenValue,
    pub line: usize,
    pub file_id: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TokenValue {
    Identifier(String),
    OpenParen,
    CloseParen,
    Plus,
    Minus,
    Value(u16, u8),
    Comma,
    Dot,
    Colon,
    Amp,
    Asterisk,
    At,
    NewLine,
    EOF,
    Directive(String),
}
