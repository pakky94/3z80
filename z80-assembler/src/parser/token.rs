#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Token {
    Label(String),
    Identifier(String),
    Address(u16),
    ShortValue(u8),
    WideValue(u16),
    Comma,
    NewLine,
    EOF,
}
