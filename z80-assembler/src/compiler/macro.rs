use crate::parser::Token;

#[derive(Debug)]
pub struct Macro {
    pub name: String,
    pub args: Vec<Vec<Token>>,
    pub tokens: Vec<Token>,
}
