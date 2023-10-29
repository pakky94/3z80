#[derive(Debug, Eq, PartialEq)]
pub enum Token<'a> {
    Label(&'a str),
    Identifier(&'a str),
    Address(u16),
    ShortValue(u8),
    WideValue(u16),
    Comma,
    NewLine,
    EOF
}
