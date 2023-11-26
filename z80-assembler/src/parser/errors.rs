use crate::parser::token::TokenValue;

#[derive(Debug, Eq, PartialEq)]
pub enum ParseError {
    UnexpectedChar(char, usize),
    UnexpectedEOF(usize),
    UnexpectedToken(UnexpectedToken),
}

#[derive(Debug, Eq, PartialEq)]
pub struct UnexpectedToken {
    pub expected: TokenValue,
    pub actual: TokenValue,
    pub line: usize,
    pub file_id: usize,
    pub char: usize,
}
