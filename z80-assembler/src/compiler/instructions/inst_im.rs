use crate::compiler::instructions::common::compile_data_2;
use crate::compiler::instructions::errors::unimplemented_instr;
use crate::compiler::instructions::{CompileData, CompileError};
use crate::domain::{Argument, Instruction};

pub fn compile_im(inst: &Instruction) -> Result<CompileData, CompileError> {
    match (&inst.arg0, &inst.arg1) {
        // TODO: handle placeholders here
        (Argument::Value(0), Argument::None) => compile_data_2(0xED, 0x46),
        (Argument::Value(1), Argument::None) => compile_data_2(0xED, 0x56),
        (Argument::Value(2), Argument::None) => compile_data_2(0xED, 0x5E),
        (_, _) => unimplemented_instr(&inst),
    }
}