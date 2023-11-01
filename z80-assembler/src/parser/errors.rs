use crate::parser::token::Token;

#[derive(Debug, Eq, PartialEq)]
pub enum ParseError {
    UnexpectedChar(char, usize),
    UnexpectedEOF(usize),
    UnexpectedToken(UnexpectedToken),
}

#[derive(Debug, Eq, PartialEq)]
pub struct UnexpectedToken {
    pub expected: Token,
    pub actual: Token,
    pub line: usize,
    pub char: usize,
}
