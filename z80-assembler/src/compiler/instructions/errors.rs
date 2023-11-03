use crate::compiler::instructions::{CompileData, Placeholder};
use crate::domain::{Argument, Instruction};
use crate::parser::ParseError;

#[derive(Debug, Eq, PartialEq)]
pub struct CompileError {
    pub error: CompileErrorType,
    pub instr: Option<Instruction>,
}

#[derive(Debug, Eq, PartialEq)]
pub enum CompileErrorType {
    ParseError(ParseError),
    ExpectedShortArgument(usize, u16),
    LabelNotFound(String, usize),
    UnexpectedArgument(Argument),
    ConstantNotFound(String),
}

impl From<ParseError> for CompileError {
    fn from(error: ParseError) -> Self {
        CompileError {
            error: CompileErrorType::ParseError(error),
            instr: None,
        }
    }
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

pub fn unexpected_arguments(
    inst: &Instruction,
    arg: &Argument,
) -> Result<CompileData, CompileError> {
    Err(CompileError {
        error: CompileErrorType::UnexpectedArgument(arg.clone()),
        instr: Some(inst.clone()),
    })
}

pub fn label_not_found(ph: &Placeholder) -> CompileError {
    CompileError {
        error: CompileErrorType::LabelNotFound(ph.label.clone(), ph.line),
        instr: None,
    }
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
