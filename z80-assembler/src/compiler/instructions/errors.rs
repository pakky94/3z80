use crate::domain::Instruction;

pub fn unimplemented_instr(instr: &Instruction) -> ! {
    unimplemented!(
        "l{} - unimplemented instruction '{}' arg0: {:?} arg1: {:?}",
        instr.line,
        instr.opcode.to_uppercase(),
        instr.arg0,
        instr.arg1
    )
}
