use crate::compiler::instructions::errors::unimplemented_instr;
pub use crate::compiler::instructions::errors::{label_not_found, CompileError, CompileErrorType};
use crate::compiler::instructions::inst_ld::compile_ld;
use crate::domain::Instruction;

mod common;
mod errors;
mod inst_ld;

pub struct CompileData {
    pub len: u8,
    pub data: [u8; 4],
    pub placeholder: Option<Placeholder>,
}

pub struct Placeholder {
    pub idx: usize,
    pub label: String,
    pub size: u8,
    pub ph_type: PlaceholderType,
    pub line: usize,
}

pub enum PlaceholderType {
    Value,
    Address,
}

pub fn compile_instruction(inst: &Instruction, idx: usize) -> Result<CompileData, CompileError> {
    match inst.opcode.as_str() {
        "ld" => compile_ld(inst, idx),
        _ => unimplemented_instr(&inst),
    }
}
