use crate::compiler::instructions::common::*;
use crate::compiler::instructions::CompileResult;
use crate::domain::{Argument, Instruction};

const LD_R_R: u8 = 0b01000000;
const LD_R_N: u8 = 0b00000110;

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
                compile_data_2(opcode, 0u8, placeholder(idx + 1, label))
            }
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    }
}
