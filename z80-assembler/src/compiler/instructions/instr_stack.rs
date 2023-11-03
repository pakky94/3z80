use crate::compiler::instructions::common::{compile_data_1, compile_data_2, to_2bit_code};
use crate::compiler::instructions::errors::unimplemented_instr;
use crate::compiler::instructions::{CompileData, CompileError};
use crate::domain::enums::WideReg;
use crate::domain::{Argument, Instruction};

pub fn compile_push(inst: &Instruction) -> Result<CompileData, CompileError> {
    match (&inst.arg0, &inst.arg1) {
        (Argument::WideReg(WideReg::IX), Argument::None) => compile_data_2(0xDD, 0xE5),
        (Argument::WideReg(WideReg::IY), Argument::None) => compile_data_2(0xFD, 0xE5),
        (Argument::WideReg(wr), Argument::None) => {
            compile_data_1(0b11000101 | (to_2bit_code(*wr)? << 4))
        }
        (_, _) => unimplemented_instr(&inst),
    }
}

pub fn compile_pop(inst: &Instruction) -> Result<CompileData, CompileError> {
    match (&inst.arg0, &inst.arg1) {
        (Argument::WideReg(WideReg::IX), Argument::None) => compile_data_2(0xDD, 0xE1),
        (Argument::WideReg(WideReg::IY), Argument::None) => compile_data_2(0xFD, 0xE1),
        (Argument::WideReg(wr), Argument::None) => {
            compile_data_1(0b11000001 | (to_2bit_code(*wr)? << 4))
        }
        (_, _) => unimplemented_instr(&inst),
    }
}
