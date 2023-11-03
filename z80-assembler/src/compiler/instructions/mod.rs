use crate::compiler::instructions::common::{
    compile_data_1, compile_data_2, compile_data_3, high_byte, low_byte, to_cond_code, update_ph,
};
use crate::compiler::instructions::errors::{
    guard_values_short, unexpected_arguments, unimplemented_instr,
};
pub use crate::compiler::instructions::errors::{label_not_found, CompileError, CompileErrorType};
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

pub fn compile_instruction(
    inst: &Instruction,
    p0: isize,
    p1: isize,
    phs: &mut Vec<Placeholder>,
) -> Result<CompileData, CompileError> {
    match inst.opcode.as_str() {
        "ld" => compile_ld(inst, p0, p1, phs),
        // Exchange, Block Transfer, and Search Group
        "ex" => compile_ex(inst),
        "exx" => inst_no_args(compile_data_1(0b11011001), inst),
        "ldi" => inst_no_args(compile_data_2(0b11101101, 0b10100000), inst),
        "ldir" => inst_no_args(compile_data_2(0b11101101, 0b10110000), inst),
        "ldd" => inst_no_args(compile_data_2(0b11101101, 0b10101000), inst),
        "lddr" => inst_no_args(compile_data_2(0b11101101, 0b10111000), inst),
        "cpd" => inst_no_args(compile_data_2(0b11101101, 0b10101001), inst),
        "cpdr" => inst_no_args(compile_data_2(0b11101101, 0b10111001), inst),
        "cpi" => inst_no_args(compile_data_2(0b11101101, 0b10100001), inst),
        "cpir" => inst_no_args(compile_data_2(0b11101101, 0b10110001), inst),
        // General-Purpose Arithmetic and CPU Control Groups
        "daa" => inst_no_args(compile_data_1(0b00100111), inst),
        "cpl" => inst_no_args(compile_data_1(0b00101111), inst),
        "neg" => inst_no_args(compile_data_2(0b11101101, 0b01000100), inst),
        "ccf" => inst_no_args(compile_data_1(0b00111111), inst),
        "scf" => inst_no_args(compile_data_1(0b00110111), inst),
        "nop" => inst_no_args(compile_data_1(0b00000000), inst),
        "halt" => inst_no_args(compile_data_1(0b01110110), inst),
        "di" => inst_no_args(compile_data_1(0b11110011), inst),
        "ei" => inst_no_args(compile_data_1(0b11111011), inst),
        "im" => compile_im(inst, p0, phs),
        // Call and Return Group
        "call" => compile_call(&inst, p0, p1, phs),
        "ret" => compile_ret(&inst),
        "reti" => inst_no_args(compile_data_2(0xED, 0x4D), inst),
        "retn" => inst_no_args(compile_data_2(0xED, 0x45), inst),
        "rst" => compile_rst(&inst),
        // Jump Group
        "jp" => todo!(),
        "jr" => todo!(),
        "djnz" => todo!(),
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

fn compile_call(
    inst: &Instruction,
    p0: isize,
    p1: isize,
    phs: &mut Vec<Placeholder>,
) -> Result<CompileData, CompileError> {
    match (&inst.arg0, &inst.arg1) {
        (Argument::Value(val), Argument::None) => {
            update_ph(p0, 1, 2, phs);
            compile_data_3(0xCD, low_byte(*val), high_byte(*val))
        }
        (Argument::Condition(c), Argument::Value(val)) => {
            update_ph(p1, 1, 2, phs);
            compile_data_3(
                0b11000100 | (to_cond_code(*c)? << 3),
                low_byte(*val),
                high_byte(*val),
            )
        }
        (_, _) => unimplemented_instr(&inst),
    }
}

fn compile_ret(inst: &Instruction) -> Result<CompileData, CompileError> {
    match (&inst.arg0, &inst.arg1) {
        (Argument::None, Argument::None) => compile_data_1(0xC9),
        (Argument::Condition(c), Argument::None) => {
            compile_data_1(0b11000000 | (to_cond_code(*c)? << 3))
        }
        (_, _) => unimplemented_instr(&inst),
    }
}

fn compile_rst(inst: &&Instruction) -> Result<CompileData, CompileError> {
    match (&inst.arg0, &inst.arg1) {
        (Argument::Value(val), Argument::None) => {
            // TODO: better error message?? also placeholder should not be allowed here
            guard_values_short(*val, 0, || {
                let val = *val as u8 & 0b00111000;
                compile_data_1(0b11000111 | val)
            })
        }
        (_, _) => unimplemented_instr(&inst),
    }
}
