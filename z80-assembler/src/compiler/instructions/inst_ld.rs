use crate::compiler::instructions::common::*;
use crate::compiler::instructions::errors::{guard_values_short, unimplemented_instr};
use crate::compiler::instructions::{CompileData, CompileError, Placeholder, PlaceholderType};
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

pub fn compile_ld(
    inst: &Instruction,
    p0: isize,
    p1: isize,
    phs: &mut Vec<Placeholder>,
) -> Result<CompileData, CompileError> {
    match (&inst.arg0, &inst.arg1) {
        // 8-bit
        (Argument::ShortReg(ShortReg::A), Argument::ShortReg(ShortReg::I)) => {
            compile_data_2(0xED, 0x57)
        }
        (Argument::ShortReg(ShortReg::A), Argument::ShortReg(ShortReg::R)) => {
            compile_data_2(0xED, 0x5F)
        }
        (Argument::ShortReg(ShortReg::I), Argument::ShortReg(ShortReg::A)) => {
            compile_data_2(0xED, 0x47)
        }
        (Argument::ShortReg(ShortReg::R), Argument::ShortReg(ShortReg::A)) => {
            compile_data_2(0xED, 0x4F)
        }
        (Argument::ShortReg(sr0), Argument::ShortReg(sr1)) => {
            let opcode = LD_R_R | (to_3bit_code(*sr0)? << 3) | to_3bit_code(*sr1)?;
            compile_data_1(opcode)
        }
        (Argument::ShortReg(sr0), Argument::Value(val)) => guard_values_short(0, *val, || {
            update_ph(p1, 1, PlaceholderType::ShortValue, phs);
            let opcode = LD_R_N | (to_3bit_code(*sr0)? << 3);
            compile_data_2(opcode, *val as u8)
        }),
        (Argument::ShortReg(sr0), Argument::WideRegAddress(WideReg::HL)) => {
            let opcode = LD_R_HL | (to_3bit_code(*sr0)? << 3);
            compile_data_1(opcode)
        }
        (Argument::ShortReg(sr0), Argument::RegOffsetAddress(WideReg::IX, offset)) => {
            guard_values_short(0, *offset, || {
                let o1 = LD_R_IX_1 | (to_3bit_code(*sr0)? << 3);
                compile_data_3(LD_R_IX_0, o1, *offset as u8)
            })
        }
        (Argument::ShortReg(sr0), Argument::RegOffsetAddress(WideReg::IY, offset)) => {
            guard_values_short(0, *offset, || {
                let o1 = LD_R_IY_1 | (to_3bit_code(*sr0)? << 3);
                compile_data_3(LD_R_IY_0, o1, *offset as u8)
            })
        }
        (Argument::WideRegAddress(WideReg::HL), Argument::ShortReg(sr)) => {
            compile_data_1(0b01110000 | (to_3bit_code(*sr)?))
        }
        (Argument::RegOffsetAddress(WideReg::IX, offset), Argument::ShortReg(sr)) => {
            guard_values_short(*offset, 0, || {
                compile_data_3(0xDD, 0b01110000 | (to_3bit_code(*sr)?), *offset as u8)
            })
        }
        (Argument::RegOffsetAddress(WideReg::IY, offset), Argument::ShortReg(sr)) => {
            guard_values_short(*offset, 0, || {
                compile_data_3(0xFD, 0b01110000 | (to_3bit_code(*sr)?), *offset as u8)
            })
        }
        (Argument::WideRegAddress(WideReg::HL), Argument::Value(val)) => {
            guard_values_short(0, *val, || {
                update_ph(p1, 1, PlaceholderType::ShortValue, phs);
                compile_data_2(0x36, *val as u8)
            })
        }
        (Argument::RegOffsetAddress(WideReg::IX, offset), Argument::Value(val)) => {
            guard_values_short(*offset, *val, || {
                update_ph(p1, 3, PlaceholderType::ShortValue, phs);
                compile_data_4(0xDD, 0x36, *offset as u8, *val as u8)
            })
        }
        (Argument::RegOffsetAddress(WideReg::IY, offset), Argument::Value(val)) => {
            guard_values_short(*offset, *val, || {
                update_ph(p1, 3, PlaceholderType::ShortValue, phs);
                compile_data_4(LD_IX_N_0, 0x36, *offset as u8, *val as u8)
            })
        }
        (Argument::ShortReg(ShortReg::A), Argument::WideRegAddress(WideReg::BC)) => {
            compile_data_1(0x0A)
        }
        (Argument::ShortReg(ShortReg::A), Argument::WideRegAddress(WideReg::DE)) => {
            compile_data_1(0x1A)
        }
        (Argument::ShortReg(ShortReg::A), Argument::DirectAddress(addr)) => {
            update_ph(p1, 1, PlaceholderType::AbsAddress, phs);
            compile_data_3(LD_A_NN, low_byte(*addr), high_byte(*addr))
        }
        (Argument::WideRegAddress(WideReg::BC), Argument::ShortReg(ShortReg::A)) => {
            compile_data_1(0x02)
        }
        (Argument::WideRegAddress(WideReg::DE), Argument::ShortReg(ShortReg::A)) => {
            compile_data_1(0x12)
        }
        (Argument::DirectAddress(addr), Argument::ShortReg(ShortReg::A)) => {
            update_ph(p0, 1, PlaceholderType::AbsAddress, phs);
            compile_data_3(0x12, low_byte(*addr), high_byte(*addr))
        }

        // 16-bit
        (Argument::WideReg(WideReg::IX), Argument::Value(val)) => {
            update_ph(p1, 2, PlaceholderType::WideValue, phs);
            compile_data_4(0xDD, 0x21, low_byte(*val), high_byte(*val))
        }
        (Argument::WideReg(WideReg::IY), Argument::Value(val)) => {
            update_ph(p1, 2, PlaceholderType::WideValue, phs);
            compile_data_4(0xFD, 0x21, low_byte(*val), high_byte(*val))
        }
        (Argument::WideReg(wr), Argument::Value(val)) => {
            update_ph(p1, 1, PlaceholderType::WideValue, phs);
            let opcode = 0b00000001 | (to_2bit_code(*wr)? << 4);
            compile_data_3(opcode, low_byte(*val), high_byte(*val))
        }
        (Argument::WideReg(WideReg::HL), Argument::DirectAddress(addr)) => {
            update_ph(p1, 1, PlaceholderType::AbsAddress, phs);
            compile_data_3(0x2A, low_byte(*addr), high_byte(*addr))
        }
        (Argument::WideReg(WideReg::IX), Argument::DirectAddress(addr)) => {
            update_ph(p1, 2, PlaceholderType::AbsAddress, phs);
            compile_data_4(0xDD, 0x2A, low_byte(*addr), high_byte(*addr))
        }
        (Argument::WideReg(WideReg::IY), Argument::DirectAddress(addr)) => {
            update_ph(p1, 2, PlaceholderType::AbsAddress, phs);
            compile_data_4(0xFD, 0x2A, low_byte(*addr), high_byte(*addr))
        }
        (Argument::WideReg(wr), Argument::DirectAddress(addr)) => {
            update_ph(p1, 2, PlaceholderType::AbsAddress, phs);
            compile_data_4(
                0xED,
                0b01001011 | (to_2bit_code(*wr)? << 4),
                low_byte(*addr),
                high_byte(*addr),
            )
        }
        (Argument::DirectAddress(addr), Argument::WideReg(WideReg::HL)) => {
            update_ph(p0, 1, PlaceholderType::AbsAddress, phs);
            compile_data_3(0x22, low_byte(*addr), high_byte(*addr))
        }
        (Argument::DirectAddress(addr), Argument::WideReg(WideReg::IX)) => {
            update_ph(p0, 2, PlaceholderType::AbsAddress, phs);
            compile_data_4(0xDD, 0x22, low_byte(*addr), high_byte(*addr))
        }
        (Argument::DirectAddress(addr), Argument::WideReg(WideReg::IY)) => {
            update_ph(p0, 2, PlaceholderType::AbsAddress, phs);
            compile_data_4(0xFD, 0x22, low_byte(*addr), high_byte(*addr))
        }
        (Argument::DirectAddress(addr), Argument::WideReg(wr)) => {
            update_ph(p0, 2, PlaceholderType::AbsAddress, phs);
            compile_data_4(
                0xED,
                0b01000011 | (to_2bit_code(*wr)? << 4),
                low_byte(*addr),
                high_byte(*addr),
            )
        }
        (Argument::WideReg(WideReg::SP), Argument::WideReg(WideReg::HL)) => compile_data_1(0xF9),
        (Argument::WideReg(WideReg::SP), Argument::WideReg(WideReg::IX)) => {
            compile_data_2(0xDD, 0xF9)
        }
        (Argument::WideReg(WideReg::SP), Argument::WideReg(WideReg::IY)) => {
            compile_data_2(0xFD, 0xF9)
        }

        (_, _) => unimplemented_instr(&inst),
    }
}
