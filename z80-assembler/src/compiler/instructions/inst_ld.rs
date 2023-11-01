use crate::compiler::instructions::common::*;
use crate::compiler::instructions::errors::{guard_values_short, unimplemented_instr};
use crate::compiler::instructions::{CompileData, CompileError};
use crate::domain::enums::{ShortReg, WideReg};
use crate::domain::{Argument, Instruction};

const LD_R_R: u8 = 0b01000000;
const LD_R_N: u8 = 0b00000110;
const LD_A_NN: u8 = 0b00111010;
const LD_R_HL: u8 = 0b01000110;
const LD_R_IX_0: u8 = 0b11011101;
const LD_R_IX_1: u8 = 0b01000110;
const LD_R_IY_0: u8 = 0b11111101;
const LD_R_IY_1: u8 = 0b01000110;
const LD_IX_N_0: u8 = 0b11011101;
const LD_IX_N_1: u8 = 0b00110110;

const LD_DD_NN: u8 = 0b00000001;
const LD_IX_NN_0: u8 = 0b11011101;
const LD_IX_NN_1: u8 = 0b00100001;

pub fn compile_ld(inst: &Instruction, idx: usize) -> Result<CompileData, CompileError> {
    match (&inst.arg0, &inst.arg1) {
        (Argument::ShortReg(sr0), Argument::ShortReg(sr1)) => {
            let opcode = LD_R_R | (to_3bit_code(*sr0)? << 3) | to_3bit_code(*sr1)?;
            compile_data_1(opcode, None)
        }
        (Argument::ShortReg(sr0), Argument::Value(val)) => guard_values_short(0, *val, || {
            let opcode = LD_R_N | (to_3bit_code(*sr0)? << 3);
            compile_data_2(opcode, *val as u8, None)
        }),
        (Argument::ShortReg(sr0), Argument::LabelValue(label)) => {
            let opcode = LD_R_N | (to_3bit_code(*sr0)? << 3);
            compile_data_2(opcode, 0, ph_value(idx + 1, label.clone(), inst.line))
        }
        (Argument::ShortReg(ShortReg::A), Argument::RegAddress(WideReg::BC)) => {
            compile_data_1(0b00001010, None)
        }
        (Argument::ShortReg(ShortReg::A), Argument::RegAddress(WideReg::DE)) => {
            compile_data_1(0b00011010, None)
        }
        (Argument::ShortReg(ShortReg::A), Argument::LabelAddress(label)) => {
            compile_data_3(LD_A_NN, 0, 0, ph_addr(idx + 1, label.clone(), inst.line))
        }
        (Argument::ShortReg(ShortReg::A), Argument::DirectAddress(addr)) => {
            compile_data_3(LD_A_NN, low_byte(*addr), high_byte(*addr), None)
        }
        (Argument::ShortReg(sr0), Argument::RegAddress(WideReg::HL)) => {
            let opcode = LD_R_HL | (to_3bit_code(*sr0)? << 3);
            compile_data_1(opcode, None)
        }
        (Argument::ShortReg(sr0), Argument::RegOffsetAddress(WideReg::IX, offset)) => {
            guard_values_short(0, *offset, || {
                let o1 = LD_R_IX_1 | (to_3bit_code(*sr0)? << 3);
                compile_data_3(LD_R_IX_0, o1, *offset as u8, None)
            })
        }
        (Argument::ShortReg(sr0), Argument::RegOffsetAddress(WideReg::IY, offset)) => {
            guard_values_short(0, *offset, || {
                let o1 = LD_R_IY_1 | (to_3bit_code(*sr0)? << 3);
                compile_data_3(LD_R_IY_0, o1, *offset as u8, None)
            })
        }
        (Argument::WideReg(WideReg::IX), Argument::Value(val)) => compile_data_4(
            LD_IX_NN_0,
            LD_IX_NN_1,
            low_byte(*val),
            high_byte(*val),
            None,
        ),
        (Argument::WideReg(wr), Argument::Value(val)) => {
            let opcode = LD_DD_NN | (to_2bit_code(*wr)? << 4);
            compile_data_3(opcode, low_byte(*val), high_byte(*val), None)
        }
        (Argument::RegOffsetAddress(WideReg::IX, offset), Argument::Value(val)) => {
            guard_values_short(*offset, *val, || {
                compile_data_4(LD_IX_N_0, LD_IX_N_1, *offset as u8, *val as u8, None)
            })
        }
        (_, _) => unimplemented_instr(&inst),
    }
}
