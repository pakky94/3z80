use crate::parser::token::Token;

#[derive(Debug)]
pub enum ParseError {
    UnexpectedChar(char, usize),
    UnexpectedEOF(usize),
    UnexpectedToken(UnexpectedToken),
}

#[derive(Debug)]
pub struct UnexpectedToken {
    pub expected: Token,
    pub actual: Token,
    pub line: usize,
    pub char: usize,
}
