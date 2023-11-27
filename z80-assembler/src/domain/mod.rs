use crate::domain::enums::{Condition, ShortReg, WideReg};
use crate::parser::Token;

pub mod conditions;
pub mod enums;
pub mod register;

#[derive(Debug, Eq, PartialEq)]
pub enum ParseItem {
    Label(Label),
    Instruction(Instruction),
    Data(Vec<u8>),
    Constant(Constant),
    Directive(String, Vec<Token>),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Instruction {
    pub opcode: String,
    pub arg0: Argument,
    pub arg1: Argument,
    pub line: usize,
    pub file_id: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Argument {
    None,
    ShortReg(ShortReg),
    WideReg(WideReg),
    Value(u16),
    LabelValue(String),
    DirectAddress(u16),
    LabelAddress(String),
    ShortRegAddress(ShortReg),
    WideRegAddress(WideReg),
    RegOffsetAddress(WideReg, u16),
    Condition(Condition),
    Constant(String),
}

#[derive(Debug, Eq, PartialEq)]
pub struct Label {
    pub name: String,
    pub line: usize,
    pub file_id: usize,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Constant {
    pub name: String,
    pub value: u16,
}
