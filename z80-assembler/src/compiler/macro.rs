use crate::parser::Token;

#[derive(Debug)]
pub struct Macro {
    pub name: String,
    pub args: Vec<String>,
    pub tokens: Vec<Token>,
}
