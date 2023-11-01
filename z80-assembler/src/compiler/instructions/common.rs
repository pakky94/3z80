use crate::compiler::instructions::{CompileData, CompileResult, Placeholder};
use crate::domain::enums::ShortReg;

pub fn to_3bit_code(sr: ShortReg) -> u8 {
    match sr {
        ShortReg::A => 0b111,
        ShortReg::B => 0b000,
        ShortReg::C => 0b001,
        ShortReg::D => 0b010,
        ShortReg::E => 0b011,
        ShortReg::H => 0b100,
        ShortReg::L => 0b101,
    }
}

pub fn compile_data_1(b0: u8, placeholder: Option<Placeholder>) -> CompileResult {
    CompileResult::Data(CompileData {
        len: 1,
        data: [b0, 0, 0, 0],
        placeholder,
    })
}

pub fn compile_data_2(b0: u8, b1: u8, placeholder: Option<Placeholder>) -> CompileResult {
    CompileResult::Data(CompileData {
        len: 2,
        data: [b0, b1, 0, 0],
        placeholder,
    })
}

pub fn placeholder(idx: usize, label: String) -> Option<Placeholder> {
    Some(Placeholder {
        idx,
        label,
    })
}