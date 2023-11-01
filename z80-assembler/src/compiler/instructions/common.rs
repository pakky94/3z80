use crate::compiler::instructions::{CompileData, CompileResult, Placeholder, PlaceholderType};
use crate::domain::enums::ShortReg;
use crate::domain::Instruction;

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

pub fn compile_data_3(b0: u8, b1: u8, b2: u8, placeholder: Option<Placeholder>) -> CompileResult {
    CompileResult::Data(CompileData {
        len: 3,
        data: [b0, b1, b2, 0],
        placeholder,
    })
}

pub fn compile_data_4(
    b0: u8,
    b1: u8,
    b2: u8,
    b4: u8,
    placeholder: Option<Placeholder>,
) -> CompileResult {
    CompileResult::Data(CompileData {
        len: 4,
        data: [b0, b1, b2, b4],
        placeholder,
    })
}

pub fn ph_value(idx: usize, label: String) -> Option<Placeholder> {
    Some(Placeholder {
        idx,
        label,
        size: 1,
        ph_type: PlaceholderType::Value,
    })
}

pub fn ph_addr(idx: usize, label: String) -> Option<Placeholder> {
    Some(Placeholder {
        idx,
        label,
        size: 2,
        ph_type: PlaceholderType::Address,
    })
}

pub fn unimplemented_instr(instr: &Instruction) -> ! {
    unimplemented!(
        "unimplemented instruction '{}' arg0: {:?} arg1: {:?}",
        instr.opcode.to_uppercase(),
        instr.arg0,
        instr.arg1
    )
}
