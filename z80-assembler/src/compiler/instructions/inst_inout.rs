use crate::compiler::instructions::common::{compile_data_2, to_3bit_code, update_ph};
use crate::compiler::instructions::errors::{guard_values_short, unimplemented_instr};
use crate::compiler::instructions::{CompileData, CompileError, Placeholder, PlaceholderType};
use crate::domain::enums::ShortReg;
use crate::domain::{Argument, Instruction};

pub fn compile_in(
    inst: &Instruction,
    _p0: isize,
    p1: isize,
    phs: &mut Vec<Placeholder>,
) -> Result<CompileData, CompileError> {
    match (&inst.arg0, &inst.arg1) {
        (Argument::ShortReg(ShortReg::A), Argument::Value(val)) => {
            guard_values_short(0, *val, || {
                update_ph(p1, 1, PlaceholderType::ShortValue, phs);
                compile_data_2(0xDB, *val as u8)
            })
        }
        (Argument::ShortReg(sr), Argument::ShortRegAddress(ShortReg::C)) => {
            compile_data_2(0xED, 0b01000000 | (to_3bit_code(*sr)? << 3))
        }
        (_, _) => unimplemented_instr(&inst),
    }
}

pub fn compile_out(
    inst: &Instruction,
    _p0: isize,
    p1: isize,
    phs: &mut Vec<Placeholder>,
) -> Result<CompileData, CompileError> {
    match (&inst.arg0, &inst.arg1) {
        (Argument::Value(val), Argument::ShortReg(ShortReg::A)) => {
            guard_values_short(0, *val, || {
                update_ph(p1, 1, PlaceholderType::ShortValue, phs);
                compile_data_2(0xD3, *val as u8)
            })
        }
        (Argument::ShortRegAddress(ShortReg::C), Argument::ShortReg(sr)) => {
            compile_data_2(0xED, 0b01000001 | (to_3bit_code(*sr)? << 3))
        }
        (_, _) => unimplemented_instr(&inst),
    }
}
