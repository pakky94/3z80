use crate::compiler::instructions::{CompileData, CompileError, CompileErrorType, Placeholder, PlaceholderType};
use crate::domain::enums::{Condition, ShortReg, WideReg};
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

pub fn to_cond_code(c: Condition) -> Result<u8, CompileError> {
    match c {
        Condition::NZ => Ok(0b000),
        Condition::Z => Ok(0b001),
        Condition::NC => Ok(0b010),
        Condition::C => Ok(0b011),
        Condition::PO => Ok(0b100),
        Condition::PE => Ok(0b101),
        Condition::P => Ok(0b110),
        Condition::M => Ok(0b111),
    }
}

pub fn low_byte(val: u16) -> u8 {
    (val % 256) as u8
}

pub fn high_byte(val: u16) -> u8 {
    (val / 256) as u8
}

pub fn compile_data_1(b0: u8) -> Result<CompileData, CompileError> {
    Ok(CompileData {
        len: 1,
        data: [b0, 0, 0, 0],
    })
}

pub fn compile_data_2(b0: u8, b1: u8) -> Result<CompileData, CompileError> {
    Ok(CompileData {
        len: 2,
        data: [b0, b1, 0, 0],
    })
}

pub fn compile_data_3(b0: u8, b1: u8, b2: u8) -> Result<CompileData, CompileError> {
    Ok(CompileData {
        len: 3,
        data: [b0, b1, b2, 0],
    })
}

pub fn compile_data_4(b0: u8, b1: u8, b2: u8, b4: u8) -> Result<CompileData, CompileError> {
    Ok(CompileData {
        len: 4,
        data: [b0, b1, b2, b4],
    })
}

pub fn update_ph(p_idx: isize, delta_idx: usize, t: PlaceholderType, phs: &mut Vec<Placeholder>) {
    if p_idx < 0 {
        return;
    }

    let p_idx = usize::try_from(p_idx).unwrap();
    phs[p_idx].idx += delta_idx;
    phs[p_idx].ph_type = t;
}
