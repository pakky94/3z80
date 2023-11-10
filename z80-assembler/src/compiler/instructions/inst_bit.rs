use crate::compiler::instructions::common::{compile_data_2, compile_data_4, to_3bit_code};
use crate::compiler::instructions::errors::{guard_values_short, unimplemented_instr};
use crate::compiler::instructions::{CompileData, CompileError, CompileErrorType};
use crate::domain::enums::WideReg;
use crate::domain::{Argument, Instruction};

pub fn inst_bit(inst: &Instruction, p0: isize) -> Result<CompileData, CompileError> {
    bit_inst(inst, p0, BIT_CODES)
}

pub fn inst_set(inst: &Instruction, p0: isize) -> Result<CompileData, CompileError> {
    bit_inst(inst, p0, SET_CODES)
}

pub fn inst_res(inst: &Instruction, p0: isize) -> Result<CompileData, CompileError> {
    bit_inst(inst, p0, RES_CODES)
}

fn bit_inst(inst: &Instruction, p: isize, codes: BitGrCodes) -> Result<CompileData, CompileError> {
    match (&inst.arg0, &inst.arg1) {
        (Argument::Value(bit), Argument::ShortReg(sr)) => guard_values_3bit(*bit, p, || {
            let opcode = codes.r_1 | ((*bit as u8) << 3) | to_3bit_code(*sr)?;
            compile_data_2(0xCB, opcode)
        }),
        (Argument::Value(bit), Argument::WideRegAddress(WideReg::HL)) => {
            guard_values_3bit(*bit, p, || {
                let opcode = codes.hl_1 | ((*bit as u8) << 3);
                compile_data_2(0xCB, opcode)
            })
        }
        (Argument::Value(bit), Argument::RegOffsetAddress(WideReg::IX, offset)) => {
            guard_values_3bit(*bit, p, || {
                guard_values_short(0, *offset, || {
                    let opcode = codes.r_1 | ((*bit as u8) << 3);
                    compile_data_4(0xDD, 0xCB, *offset as u8, opcode)
                })
            })
        }
        (Argument::Value(bit), Argument::RegOffsetAddress(WideReg::IY, offset)) => {
            guard_values_3bit(*bit, p, || {
                guard_values_short(0, *offset, || {
                    let opcode = codes.r_1 | ((*bit as u8) << 3);
                    compile_data_4(0xFD, 0xCB, *offset as u8, opcode)
                })
            })
        }
        _ => unimplemented_instr(inst),
    }
}

fn guard_values_3bit<T>(val1: u16, p: isize, f: T) -> Result<CompileData, CompileError>
where
    T: FnOnce() -> Result<CompileData, CompileError>,
{
    if val1 >= 8 {
        Err(CompileError {
            error: CompileErrorType::ExpectedBitArgument(0, val1),
            instr: None,
        })
    } else if p >= 0 {
        Err(CompileError {
            error: CompileErrorType::ExpectedBitArgument(0, val1),
            instr: None,
        })
    } else {
        f()
    }
}

struct BitGrCodes {
    r_1: u8,
    hl_1: u8,
}

const BIT_CODES: BitGrCodes = BitGrCodes {
    r_1: 0b01000000,
    hl_1: 0b01000110,
};

const SET_CODES: BitGrCodes = BitGrCodes {
    r_1: 0b11000000,
    hl_1: 0b11000110,
};

const RES_CODES: BitGrCodes = BitGrCodes {
    r_1: 0b10000000,
    hl_1: 0b10000110,
};
