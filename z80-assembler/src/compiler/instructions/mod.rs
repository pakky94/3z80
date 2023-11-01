use crate::compiler::instructions::inst_ld::compile_ld;
use crate::domain::Instruction;

mod common;
mod inst_ld;

pub enum CompileResult {
    Data(CompileData),
    CompileError,
}

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
}

pub enum PlaceholderType {
    Value,
    Address,
}

pub fn compile_instruction(inst: Instruction, idx: usize) -> CompileResult {
    match inst.opcode.as_str() {
        "ld" => compile_ld(inst, idx),
        _ => unimplemented!(),
    }
}
