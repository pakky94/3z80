use crate::compiler::instructions::common::{
    compile_data_1, compile_data_2, compile_data_3, compile_data_4, high_byte, low_byte,
};
pub use crate::compiler::instructions::errors::{label_not_found, CompileError, CompileErrorType};
use crate::compiler::instructions::errors::{unexpected_arguments, unimplemented_instr};
use crate::compiler::instructions::inst_ex::compile_ex;
use crate::compiler::instructions::inst_im::compile_im;
use crate::compiler::instructions::inst_ld::compile_ld;
use crate::domain::{Argument, Instruction};

pub mod common;
mod errors;
mod inst_ex;
mod inst_im;
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
        // Exchange, Block Transfer, and Search Group
        "ex" => compile_ex(inst, idx),
        "exx" => inst_no_args(compile_data_1(0b11011001, None), inst),
        "ldi" => inst_no_args(compile_data_2(0b11101101, 0b10100000, None), inst),
        "ldir" => inst_no_args(compile_data_2(0b11101101, 0b10110000, None), inst),
        "ldd" => inst_no_args(compile_data_2(0b11101101, 0b10101000, None), inst),
        "lddr" => inst_no_args(compile_data_2(0b11101101, 0b10111000, None), inst),
        "cpd" => inst_no_args(compile_data_2(0b11101101, 0b10101001, None), inst),
        "cpdr" => inst_no_args(compile_data_2(0b11101101, 0b10111001, None), inst),
        "cpi" => inst_no_args(compile_data_2(0b11101101, 0b10100001, None), inst),
        "cpir" => inst_no_args(compile_data_2(0b11101101, 0b10110001, None), inst),
        // General-Purpose Arithmetic and CPU Control Groups
        "daa" => inst_no_args(compile_data_1(0b00100111, None), inst),
        "cpl" => inst_no_args(compile_data_1(0b00101111, None), inst),
        "neg" => inst_no_args(compile_data_2(0b11101101, 0b01000100, None), inst),
        "ccf" => inst_no_args(compile_data_1(0b00111111, None), inst),
        "scf" => inst_no_args(compile_data_1(0b00110111, None), inst),
        "nop" => inst_no_args(compile_data_1(0b00000000, None), inst),
        "halt" => inst_no_args(compile_data_1(0b01110110, None), inst),
        "di" => inst_no_args(compile_data_1(0b11110011, None), inst),
        "ei" => inst_no_args(compile_data_1(0b11111011, None), inst),
        "im" => compile_im(inst, idx),
        // Call and Return Group
        "call" => todo!(),
        "ret" => todo!(),
        "reti" => inst_no_args(compile_data_2(0xED, 0x4D, None), inst),
        "retn" => inst_no_args(compile_data_2(0xED, 0x45, None), inst),
        _ => unimplemented_instr(&inst),
    }
}

fn inst_no_args(
    data: Result<CompileData, CompileError>,
    inst: &Instruction,
) -> Result<CompileData, CompileError> {
    if let (Argument::None, Argument::None) = (&inst.arg0, &inst.arg1) {
        data
    } else {
        if let Argument::None = &inst.arg0 {
            unexpected_arguments(&inst, &inst.arg1)
        } else {
            unexpected_arguments(&inst, &inst.arg0)
        }
    }
}

fn compile_call(inst: &Instruction, _: usize) -> Result<CompileData, CompileError> {
    match (&inst.arg0, &inst.arg1) {
        (Argument::Value(val), Argument::None) => {
            compile_data_3(0xCD, low_byte(*val), high_byte(*val), None)
        }
        (_, _) => unimplemented_instr(&inst),
    }
}
