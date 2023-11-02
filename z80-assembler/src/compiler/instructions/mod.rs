use crate::compiler::instructions::common::{compile_data_1, compile_data_2};
use crate::compiler::instructions::errors::unimplemented_instr;
pub use crate::compiler::instructions::errors::{label_not_found, CompileError, CompileErrorType};
use crate::compiler::instructions::inst_ex::compile_ex;
use crate::compiler::instructions::inst_ld::compile_ld;
use crate::domain::{Argument, Instruction};

pub mod common;
mod errors;
mod inst_ld;
mod inst_ex;

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
        "ex" => compile_ex(inst, idx),
        "exx" => inst_no_args(compile_data_1(0b11011001, None), inst),
        "ld" => compile_ld(inst, idx),
        "ldi" => inst_no_args(compile_data_2(0b11101101, 0b10100000, None), inst),
        "ldir" => inst_no_args(compile_data_2(0b11101101, 0b10110000, None), inst),
        "ldd" => inst_no_args(compile_data_2(0b11101101, 0b10101000, None), inst),
        "lddr" => inst_no_args(compile_data_2(0b11101101, 0b10111000, None), inst),
        _ => unimplemented_instr(&inst),
    }
}

fn inst_no_args(data: Result<CompileData, CompileError>, inst: &Instruction) -> Result<CompileData, CompileError> {
    if let (Argument::None, Argument::None) = (&inst.arg0, &inst.arg1) {
        data
    } else {
        unimplemented_instr(&inst)
    }
}