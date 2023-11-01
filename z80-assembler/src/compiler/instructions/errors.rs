use crate::compiler::instructions::CompileData;
use crate::domain::Instruction;
use crate::parser::ParseError;

#[derive(Debug)]
pub struct CompileError {
    pub error: CompileErrorType,
    pub instr: Option<Instruction>,
}

#[derive(Debug)]
pub enum CompileErrorType {
    ParseError(ParseError),
    ExpectedShortArgument(usize, u16),
}

pub fn unimplemented_instr(instr: &Instruction) -> ! {
    unimplemented!(
        "l{} - unimplemented instruction '{}' arg0: {:?} arg1: {:?}",
        instr.line,
        instr.opcode.to_uppercase(),
        instr.arg0,
        instr.arg1
    )
}

pub fn guard_values_short<T>(val1: u16, val2: u16, f: T) -> Result<CompileData, CompileError>
where
    T: FnOnce() -> Result<CompileData, CompileError>,
{
    if val1 >= 256 {
        Err(CompileError {
            error: CompileErrorType::ExpectedShortArgument(0, val1),
            instr: None,
        })
    } else if val2 >= 256 {
        Err(CompileError {
            error: CompileErrorType::ExpectedShortArgument(1, val2),
            instr: None,
        })
    } else {
        f()
    }
}
