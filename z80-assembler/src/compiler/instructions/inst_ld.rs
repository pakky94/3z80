use crate::compiler::instructions::common::*;
use crate::compiler::instructions::CompileResult;
use crate::compiler::instructions::errors::unimplemented_instr;
use crate::domain::enums::WideReg;
use crate::domain::{Argument, Instruction};

const LD_R_R: u8 = 0b01000000;
const LD_R_N: u8 = 0b00000110;
const LD_R_NN: u8 = 0b00111010;
const LD_R_HL: u8 = 0b01000110;
const LD_R_IX_0: u8 = 0b11011101;
const LD_R_IX_1: u8 = 0b01000110;
const LD_R_IY_0: u8 = 0b11111101;
const LD_R_IY_1: u8 = 0b01000110;
const LD_IX_N_0: u8 = 0b11011101;
const LD_IX_N_1: u8 = 0b00110110;

pub fn compile_ld(inst: Instruction, idx: usize) -> CompileResult {
    match inst.arg0 {
        Argument::ShortReg(sr0) => match inst.arg1 {
            Argument::ShortReg(sr1) => {
                let opcode = LD_R_R | (to_3bit_code(sr0) << 3) | to_3bit_code(sr1);
                compile_data_1(opcode, None)
            }
            Argument::Value(val) => {
                if val < 256 {
                    let opcode = LD_R_N | (to_3bit_code(sr0) << 3);
                    compile_data_2(opcode, val as u8, None)
                } else {
                    unimplemented!("error handling")
                }
            }
            Argument::LabelValue(label) => {
                let opcode = LD_R_N | (to_3bit_code(sr0) << 3);
                compile_data_2(opcode, 0, ph_value(idx + 1, label))
            }
            Argument::LabelAddress(label) => compile_data_3(LD_R_NN, 0, 0, ph_addr(idx + 1, label)),
            Argument::RegAddress(WideReg::HL) => {
                let opcode = LD_R_HL | (to_3bit_code(sr0) << 3);
                compile_data_1(opcode, None)
            }
            Argument::RegOffsetAddress(WideReg::IX, offset) => {
                if offset < 256 {
                    let o1 = LD_R_IX_1 | (to_3bit_code(sr0) << 3);
                    compile_data_3(LD_R_IX_0, o1, offset as u8, None)
                } else {
                    unimplemented!("error handling")
                }
            }
            Argument::RegOffsetAddress(WideReg::IY, offset) => {
                if offset < 256 {
                    let o1 = LD_R_IY_1 | (to_3bit_code(sr0) << 3);
                    compile_data_3(LD_R_IY_0, o1, offset as u8, None)
                } else {
                    unimplemented!("error handling")
                }
            }
            _ => unimplemented_instr(&inst),
        },
        Argument::RegOffsetAddress(WideReg::IX, offset) => match inst.arg1 {
            Argument::Value(val) => {
                if offset < 256 && val < 256 {
                    compile_data_4(LD_IX_N_0, LD_IX_N_1, offset as u8, val as u8, None)
                } else {
                    unimplemented!("error handling")
                }
            }
            _ => unimplemented_instr(&inst),
        },
        _ => unimplemented_instr(&inst),
    }
}
