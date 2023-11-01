use crate::compiler::instructions::{
    CompileData, CompileError, CompileErrorType, Placeholder, PlaceholderType,
};
use crate::domain::enums::{ShortReg, WideReg};
use crate::domain::Argument;

pub fn to_3bit_code(sr: ShortReg) -> Result<u8, CompileError> {
    match sr {
        ShortReg::A => Ok(0b111),
        ShortReg::B => Ok(0b000),
        ShortReg::C => Ok(0b001),
        ShortReg::D => Ok(0b010),
        ShortReg::E => Ok(0b011),
        ShortReg::H => Ok(0b100),
        ShortReg::L => Ok(0b101),
        _ => Err(CompileError {
            error: CompileErrorType::UnexpectedArgument(Argument::ShortReg(sr)),
            instr: None,
        }),
    }
}

pub fn to_2bit_code(wr: WideReg) -> Result<u8, CompileError> {
    match wr {
        WideReg::BC => Ok(0b00),
        WideReg::DE => Ok(0b01),
        WideReg::HL => Ok(0b10),
        WideReg::SP => Ok(0b11),
        _ => Err(CompileError {
            error: CompileErrorType::UnexpectedArgument(Argument::WideReg(wr)),
            instr: None,
        }),
    }
}

pub fn low_byte(val: u16) -> u8 {
    (val % 256) as u8
}

pub fn high_byte(val: u16) -> u8 {
    (val / 256) as u8
}

pub fn compile_data_1(
    b0: u8,
    placeholder: Option<Placeholder>,
) -> Result<CompileData, CompileError> {
    Ok(CompileData {
        len: 1,
        data: [b0, 0, 0, 0],
        placeholder,
    })
}

pub fn compile_data_2(
    b0: u8,
    b1: u8,
    placeholder: Option<Placeholder>,
) -> Result<CompileData, CompileError> {
    Ok(CompileData {
        len: 2,
        data: [b0, b1, 0, 0],
        placeholder,
    })
}

pub fn compile_data_3(
    b0: u8,
    b1: u8,
    b2: u8,
    placeholder: Option<Placeholder>,
) -> Result<CompileData, CompileError> {
    Ok(CompileData {
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
) -> Result<CompileData, CompileError> {
    Ok(CompileData {
        len: 4,
        data: [b0, b1, b2, b4],
        placeholder,
    })
}

pub fn ph_value(idx: usize, label: String, line: usize) -> Option<Placeholder> {
    Some(Placeholder {
        idx,
        label,
        size: 1,
        ph_type: PlaceholderType::Value,
        line,
    })
}

pub fn ph_addr(idx: usize, label: String, line: usize) -> Option<Placeholder> {
    Some(Placeholder {
        idx,
        label,
        size: 2,
        ph_type: PlaceholderType::Address,
        line,
    })
}
