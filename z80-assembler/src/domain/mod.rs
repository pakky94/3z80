use crate::domain::enums::{Condition, ShortReg, WideReg};

pub mod conditions;
pub mod enums;
pub mod register;

#[derive(Debug, Eq, PartialEq)]
pub enum ParseItem {
    Label(Label),
    Instruction(Instruction),
}

#[derive(Debug, Eq, PartialEq)]
pub struct Instruction {
    pub opcode: String,
    pub arg0: Argument,
    pub arg1: Argument,
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
    RegAddress(WideReg),
    RegOffsetAddress(WideReg, u16),
    Condition(Condition),
}

#[derive(Debug, Eq, PartialEq)]
pub struct Label {
    pub name: String,
    pub target: usize,
}