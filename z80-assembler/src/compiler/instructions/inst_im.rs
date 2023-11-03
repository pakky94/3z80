use crate::compiler::instructions::common::{compile_data_2, update_ph};
use crate::compiler::instructions::errors::unimplemented_instr;
use crate::compiler::instructions::{CompileData, CompileError, Placeholder};
use crate::domain::{Argument, Instruction};

pub fn compile_im(
    inst: &Instruction,
    p0: isize,
    phs: &mut Vec<Placeholder>,
) -> Result<CompileData, CompileError> {
    match (&inst.arg0, &inst.arg1) {
        (Argument::Value(0), Argument::None) => {
            update_ph(p0, 1, 1, phs);
            compile_data_2(0xED, 0x46)
        }
        (Argument::Value(1), Argument::None) => {
            update_ph(p0, 1, 1, phs);
            compile_data_2(0xED, 0x56)
        }
        (Argument::Value(2), Argument::None) => {
            update_ph(p0, 1, 1, phs);
            compile_data_2(0xED, 0x5E)
        }
        (_, _) => unimplemented_instr(&inst),
    }
}
