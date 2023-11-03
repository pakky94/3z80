use crate::compiler::instructions::common::{compile_data_1, compile_data_2};
use crate::compiler::instructions::errors::unimplemented_instr;
use crate::compiler::instructions::{CompileData, CompileError};
use crate::domain::enums::WideReg;
use crate::domain::{Argument, Instruction};

pub fn compile_ex(inst: &Instruction) -> Result<CompileData, CompileError> {
    match (&inst.arg0, &inst.arg1) {
        (Argument::WideReg(WideReg::DE), Argument::WideReg(WideReg::HL)) => {
            compile_data_1(0b11101011, None)
        }
        (Argument::WideReg(WideReg::AF), Argument::WideReg(WideReg::AFp)) => {
            compile_data_1(0b00001000, None)
        }
        (Argument::RegAddress(WideReg::SP), Argument::WideReg(WideReg::HL)) => {
            compile_data_1(0b11100011, None)
        }
        (Argument::RegAddress(WideReg::SP), Argument::WideReg(WideReg::IX)) => {
            compile_data_2(0b11011101, 0b11100011, None)
        }
        (Argument::RegAddress(WideReg::SP), Argument::WideReg(WideReg::IY)) => {
            compile_data_2(0b11111101, 0b11100011, None)
        }
        (_, _) => unimplemented_instr(&inst),
    }
}
