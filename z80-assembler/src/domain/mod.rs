use crate::domain::enums::{ShortReg, WideReg};

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

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Argument {
    None,
    ShortReg(ShortReg),
    WideReg(WideReg),
    Short(u8),
    Wide(u16),
    DirectAddress(u16),
    RegAddress(WideReg),
    RegOffsetAddress(WideReg, u8),
}

#[derive(Debug, Eq, PartialEq)]
pub struct Label {
    pub name: String,
    pub target: usize,
}
