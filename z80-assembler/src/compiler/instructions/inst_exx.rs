use crate::compiler::instructions::{CompileData, CompileError};
use crate::compiler::instructions::common::compile_data_1;
use crate::compiler::instructions::errors::unimplemented_instr;
use crate::domain::{Argument, Instruction};

pub fn compile_exx(inst: &Instruction, _: usize) -> Result<CompileData, CompileError> {
    match (&inst.arg0, &inst.arg1) {
        (Argument::None, Argument::None) => compile_data_1(0b11011001, None),
        (_, _) => unimplemented_instr(inst),
    }
}