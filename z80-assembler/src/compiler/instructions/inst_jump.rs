use crate::compiler::instructions::common::{
    compile_data_1, compile_data_2, compile_data_3, high_byte, low_byte, to_cond_code, update_ph,
};
use crate::compiler::instructions::errors::{guard_values_short, unimplemented_instr};
use crate::compiler::instructions::{CompileData, CompileError, Placeholder, PlaceholderType};
use crate::domain::enums::{Condition, WideReg};
use crate::domain::{Argument, Instruction};

pub fn inst_djnz(
    inst: &Instruction,
    p: isize,
    phs: &mut Vec<Placeholder>,
) -> Result<CompileData, CompileError> {
    match (&inst.arg0, &inst.arg1) {
        (Argument::DirectAddress(val), Argument::None) => guard_values_short(*val, 0, || {
            update_ph(p, 1, PlaceholderType::RelAddress, phs);
            compile_data_2(0x10, *val as u8)
        }),
        (_, _) => unimplemented_instr(&inst),
    }
}

pub fn inst_jp(
    inst: &Instruction,
    p0: isize,
    p1: isize,
    phs: &mut Vec<Placeholder>,
) -> Result<CompileData, CompileError> {
    match (&inst.arg0, &inst.arg1) {
        (Argument::DirectAddress(addr), Argument::None) => {
            update_ph(p0, 1, PlaceholderType::AbsAddress, phs);
            compile_data_3(0xC3, low_byte(*addr), high_byte(*addr))
        }
        (Argument::Condition(c), Argument::DirectAddress(addr)) => {
            let opcode = 0b11000010 | (to_cond_code(*c)? << 3);
            update_ph(p1, 1, PlaceholderType::AbsAddress, phs);
            compile_data_3(opcode, low_byte(*addr), high_byte(*addr))
        }
        (Argument::RegAddress(WideReg::HL), Argument::None) => compile_data_1(0xE9),
        (Argument::RegAddress(WideReg::IX), Argument::None) => compile_data_2(0xDD, 0xE9),
        (Argument::RegAddress(WideReg::IY), Argument::None) => compile_data_2(0xFD, 0xE9),
        (_, _) => unimplemented_instr(&inst),
    }
}

pub fn inst_jr(
    inst: &Instruction,
    p0: isize,
    p1: isize,
    phs: &mut Vec<Placeholder>,
) -> Result<CompileData, CompileError> {
    match (&inst.arg0, &inst.arg1) {
        (Argument::DirectAddress(addr), Argument::None) => {
            update_ph(p0, 1, PlaceholderType::AbsAddress, phs);
            compile_data_3(0x18, low_byte(*addr), high_byte(*addr))
        }
        (Argument::Condition(Condition::C), Argument::DirectAddress(val)) => {
            guard_values_short(*val, 0, || {
                update_ph(p1, 1, PlaceholderType::RelAddress, phs);
                compile_data_2(0x38, *val as u8)
            })
        }
        (Argument::Condition(Condition::NC), Argument::DirectAddress(val)) => {
            guard_values_short(*val, 0, || {
                update_ph(p1, 1, PlaceholderType::RelAddress, phs);
                compile_data_2(0x30, *val as u8)
            })
        }
        (Argument::Condition(Condition::Z), Argument::DirectAddress(val)) => {
            guard_values_short(*val, 0, || {
                update_ph(p1, 1, PlaceholderType::RelAddress, phs);
                compile_data_2(0x28, *val as u8)
            })
        }
        (Argument::Condition(Condition::NZ), Argument::DirectAddress(val)) => {
            guard_values_short(*val, 0, || {
                update_ph(p1, 1, PlaceholderType::RelAddress, phs);
                compile_data_2(0x20, *val as u8)
            })
        }
        (_, _) => unimplemented_instr(&inst),
    }
}
