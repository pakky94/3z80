use crate::compiler::instructions::common::{compile_data_2, update_ph};
use crate::compiler::instructions::errors::{guard_values_short, unimplemented_instr};
use crate::compiler::instructions::{CompileData, CompileError, Placeholder, PlaceholderType};
use crate::domain::enums::Condition;
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
    _p0: isize,
    _p1: isize,
    _phs: &mut Vec<Placeholder>,
) -> Result<CompileData, CompileError> {
    match (&inst.arg0, &inst.arg1) {
        (_, _) => unimplemented_instr(&inst),
    }
}

pub fn inst_jr(
    inst: &Instruction,
    _p0: isize,
    p1: isize,
    phs: &mut Vec<Placeholder>,
) -> Result<CompileData, CompileError> {
    match (&inst.arg0, &inst.arg1) {
        (Argument::Condition(Condition::Z), Argument::DirectAddress(val)) => {
            guard_values_short(*val, 0, || {
                update_ph(p1, 1, PlaceholderType::RelAddress, phs);
                compile_data_2(0x28, *val as u8)
            })
        }
        (_, _) => unimplemented_instr(&inst),
    }
}
