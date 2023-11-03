use crate::compiler::instructions::common::{
    compile_data_1, compile_data_2, compile_data_3, to_3bit_code, update_ph,
};
use crate::compiler::instructions::errors::{guard_values_short, unimplemented_instr};
use crate::compiler::instructions::{CompileData, CompileError, Placeholder, PlaceholderType};
use crate::domain::enums::{ShortReg, WideReg};
use crate::domain::{Argument, Instruction};

pub fn inst_add(
    inst: &Instruction,
    _p0: isize,
    p1: isize,
    phs: &mut Vec<Placeholder>,
) -> Result<CompileData, CompileError> {
    match (&inst.arg0, &inst.arg1) {
        (Argument::ShortReg(ShortReg::A), arg) => arith8(arg, p1, phs, ADD_CODES, inst),
        _ => unimplemented_instr(&inst),
    }
}

pub fn inst_adc(
    inst: &Instruction,
    _p0: isize,
    p1: isize,
    phs: &mut Vec<Placeholder>,
) -> Result<CompileData, CompileError> {
    match (&inst.arg0, &inst.arg1) {
        (Argument::ShortReg(ShortReg::A), arg) => arith8(arg, p1, phs, ADC_CODES, inst),
        _ => unimplemented_instr(&inst),
    }
}

pub fn inst_sub(
    inst: &Instruction,
    p0: isize,
    _p1: isize,
    phs: &mut Vec<Placeholder>,
) -> Result<CompileData, CompileError> {
    match (&inst.arg0, &inst.arg1) {
        (arg, Argument::None) => arith8(arg, p0, phs, SUB_CODES, inst),
        _ => unimplemented_instr(&inst),
    }
}

pub fn inst_sbc(
    inst: &Instruction,
    _p0: isize,
    p1: isize,
    phs: &mut Vec<Placeholder>,
) -> Result<CompileData, CompileError> {
    match (&inst.arg0, &inst.arg1) {
        (Argument::ShortReg(ShortReg::A), arg) => arith8(arg, p1, phs, SBC_CODES, inst),
        _ => unimplemented_instr(&inst),
    }
}

pub fn inst_and(
    inst: &Instruction,
    p0: isize,
    _p1: isize,
    phs: &mut Vec<Placeholder>,
) -> Result<CompileData, CompileError> {
    match (&inst.arg0, &inst.arg1) {
        (arg, Argument::None) => arith8(arg, p0, phs, AND_CODES, inst),
        _ => unimplemented_instr(&inst),
    }
}

pub fn inst_or(
    inst: &Instruction,
    p0: isize,
    _p1: isize,
    phs: &mut Vec<Placeholder>,
) -> Result<CompileData, CompileError> {
    match (&inst.arg0, &inst.arg1) {
        (arg, Argument::None) => arith8(arg, p0, phs, OR_CODES, inst),
        _ => unimplemented_instr(&inst),
    }
}

pub fn inst_xor(
    inst: &Instruction,
    p0: isize,
    _p1: isize,
    phs: &mut Vec<Placeholder>,
) -> Result<CompileData, CompileError> {
    match (&inst.arg0, &inst.arg1) {
        (arg, Argument::None) => arith8(arg, p0, phs, XOR_CODES, inst),
        _ => unimplemented_instr(&inst),
    }
}

pub fn inst_cp(
    inst: &Instruction,
    p0: isize,
    _p1: isize,
    phs: &mut Vec<Placeholder>,
) -> Result<CompileData, CompileError> {
    match (&inst.arg0, &inst.arg1) {
        (arg, Argument::None) => arith8(arg, p0, phs, CP_CODES, inst),
        _ => unimplemented_instr(&inst),
    }
}

pub fn inst_inc(
    inst: &Instruction,
) -> Result<CompileData, CompileError> {
    match (&inst.arg0, &inst.arg1) {
        (arg, Argument::None) => offset(arg, INC_CODES, inst),
        _ => unimplemented_instr(&inst),
    }
}

pub fn inst_dec(
    inst: &Instruction,
) -> Result<CompileData, CompileError> {
    match (&inst.arg0, &inst.arg1) {
        (arg, Argument::None) => offset(arg, DEC_CODES, inst),
        _ => unimplemented_instr(&inst),
    }
}

fn arith8(
    arg: &Argument,
    p: isize,
    phs: &mut Vec<Placeholder>,
    codes: ArithGrCodes,
    inst: &Instruction,
) -> Result<CompileData, CompileError> {
    match arg {
        Argument::ShortReg(sr) => {
            let opcode = codes.r | to_3bit_code(*sr)?;
            compile_data_1(opcode)
        }
        Argument::Value(val) => guard_values_short(0, *val, || {
            update_ph(p, 1, PlaceholderType::ShortValue, phs);
            compile_data_2(codes.n, *val as u8)
        }),
        Argument::RegAddress(WideReg::HL) => compile_data_1(codes.hl),
        Argument::RegOffsetAddress(WideReg::IX, offset) => guard_values_short(0, *offset, || {
            compile_data_3(codes.ix_d_0, codes.ix_d_1, *offset as u8)
        }),
        Argument::RegOffsetAddress(WideReg::IY, offset) => guard_values_short(0, *offset, || {
            compile_data_3(codes.iy_d_0, codes.iy_d_1, *offset as u8)
        }),
        _ => unimplemented_instr(inst),
    }
}

fn offset(
    arg: &Argument,
    codes: ArithGrCodes,
    inst: &Instruction,
) -> Result<CompileData, CompileError> {
    match arg {
        Argument::ShortReg(sr) => {
            let opcode = codes.r | to_3bit_code(*sr)?;
            compile_data_1(opcode)
        }
        Argument::RegAddress(WideReg::HL) => compile_data_1(codes.hl),
        Argument::RegOffsetAddress(WideReg::IX, offset) => guard_values_short(0, *offset, || {
            compile_data_3(codes.ix_d_0, codes.ix_d_1, *offset as u8)
        }),
        Argument::RegOffsetAddress(WideReg::IY, offset) => guard_values_short(0, *offset, || {
            compile_data_3(codes.iy_d_0, codes.iy_d_1, *offset as u8)
        }),
        _ => unimplemented_instr(inst),
    }
}

struct ArithGrCodes {
    r: u8,
    n: u8,
    hl: u8,
    ix_d_0: u8,
    ix_d_1: u8,
    iy_d_0: u8,
    iy_d_1: u8,
}

const ADD_CODES: ArithGrCodes = ArithGrCodes {
    r: 0b10000000,
    n: 0xC6,
    hl: 0x86,
    ix_d_0: 0xDD,
    ix_d_1: 0x86,
    iy_d_0: 0xFD,
    iy_d_1: 0x86,
};

const ADC_CODES: ArithGrCodes = ArithGrCodes {
    r: 0b10001000,
    n: 0xCE,
    hl: 0x8E,
    ix_d_0: 0xDD,
    ix_d_1: 0x8E,
    iy_d_0: 0xFD,
    iy_d_1: 0x8E,
};

const SUB_CODES: ArithGrCodes = ArithGrCodes {
    r: 0b10010000,
    n: 0xD6,
    hl: 0x96,
    ix_d_0: 0xDD,
    ix_d_1: 0x96,
    iy_d_0: 0xFD,
    iy_d_1: 0x96,
};

const SBC_CODES: ArithGrCodes = ArithGrCodes {
    r: 0b10011000,
    n: 0xDE,
    hl: 0x9E,
    ix_d_0: 0xDD,
    ix_d_1: 0x9E,
    iy_d_0: 0xFD,
    iy_d_1: 0x9E,
};

const AND_CODES: ArithGrCodes = ArithGrCodes {
    r: 0b10100000,
    n: 0xE6,
    hl: 0xA6,
    ix_d_0: 0xDD,
    ix_d_1: 0xA6,
    iy_d_0: 0xFD,
    iy_d_1: 0xA6,
};

const OR_CODES: ArithGrCodes = ArithGrCodes {
    r: 0b10110000,
    n: 0xF6,
    hl: 0xB6,
    ix_d_0: 0xDD,
    ix_d_1: 0xB6,
    iy_d_0: 0xFD,
    iy_d_1: 0xB6,
};

const XOR_CODES: ArithGrCodes = ArithGrCodes {
    r: 0b10101000,
    n: 0xEE,
    hl: 0xAE,
    ix_d_0: 0xDD,
    ix_d_1: 0xAE,
    iy_d_0: 0xFD,
    iy_d_1: 0xAE,
};

const CP_CODES: ArithGrCodes = ArithGrCodes {
    r: 0b10111000,
    n: 0xFE,
    hl: 0xBE,
    ix_d_0: 0xDD,
    ix_d_1: 0xBE,
    iy_d_0: 0xFD,
    iy_d_1: 0xBE,
};

const INC_CODES: ArithGrCodes = ArithGrCodes {
    r: 0b00000100,
    n: 0,
    hl: 0x34,
    ix_d_0: 0xDD,
    ix_d_1: 0x34,
    iy_d_0: 0xFD,
    iy_d_1: 0x34,
};

const DEC_CODES: ArithGrCodes = ArithGrCodes {
    r: 0b00000101,
    n: 0,
    hl: 0x35,
    ix_d_0: 0xDD,
    ix_d_1: 0x35,
    iy_d_0: 0xFD,
    iy_d_1: 0x35,
};
