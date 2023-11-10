use crate::compiler::instructions::common::{compile_data_2, compile_data_4, to_3bit_code};
use crate::compiler::instructions::errors::{guard_values_short, unimplemented_instr};
use crate::compiler::instructions::{CompileData, CompileError};
use crate::domain::enums::WideReg;
use crate::domain::{Argument, Instruction};

pub fn inst_rlc(inst: &Instruction) -> Result<CompileData, CompileError> {
    rot_shift(inst, RLC_CODES)
}

pub fn inst_rl(inst: &Instruction) -> Result<CompileData, CompileError> {
    rot_shift(inst, RL_CODES)
}

pub fn inst_rrc(inst: &Instruction) -> Result<CompileData, CompileError> {
    rot_shift(inst, RRC_CODES)
}

pub fn inst_rr(inst: &Instruction) -> Result<CompileData, CompileError> {
    rot_shift(inst, RR_CODES)
}

pub fn inst_sla(inst: &Instruction) -> Result<CompileData, CompileError> {
    rot_shift(inst, SLA_CODES)
}

pub fn inst_sra(inst: &Instruction) -> Result<CompileData, CompileError> {
    rot_shift(inst, SRA_CODES)
}

pub fn inst_srl(inst: &Instruction) -> Result<CompileData, CompileError> {
    rot_shift(inst, SRL_CODES)
}

fn rot_shift(inst: &Instruction, codes: RotShiftGrCodes) -> Result<CompileData, CompileError> {
    match (&inst.arg0, &inst.arg1) {
        (Argument::ShortReg(sr), Argument::None) => {
            let opcode = codes.r_1 | to_3bit_code(*sr)?;
            compile_data_2(codes.r_0, opcode)
        }
        (Argument::WideRegAddress(WideReg::HL), Argument::None) => {
            compile_data_2(codes.hl_0, codes.hl_1)
        }
        (Argument::RegOffsetAddress(WideReg::IX, offset), Argument::None) => {
            guard_values_short(0, *offset, || {
                compile_data_4(codes.ix_d_0, codes.ix_d_1, *offset as u8, codes.ix_d_3)
            })
        }
        (Argument::RegOffsetAddress(WideReg::IY, offset), Argument::None) => {
            guard_values_short(0, *offset, || {
                compile_data_4(codes.iy_d_0, codes.iy_d_1, *offset as u8, codes.iy_d_3)
            })
        }
        _ => unimplemented_instr(inst),
    }
}

struct RotShiftGrCodes {
    r_0: u8,
    r_1: u8,
    hl_0: u8,
    hl_1: u8,
    ix_d_0: u8,
    ix_d_1: u8,
    ix_d_3: u8,
    iy_d_0: u8,
    iy_d_1: u8,
    iy_d_3: u8,
}

const RLC_CODES: RotShiftGrCodes = RotShiftGrCodes {
    r_0: 0xCB,
    r_1: 0b00000000,
    hl_0: 0xCB,
    hl_1: 0x06,
    ix_d_0: 0xDD,
    ix_d_1: 0xCB,
    ix_d_3: 0x06,
    iy_d_0: 0xFD,
    iy_d_1: 0xCB,
    iy_d_3: 0x06,
};

const RL_CODES: RotShiftGrCodes = RotShiftGrCodes {
    r_0: 0xCB,
    r_1: 0b00010000,
    hl_0: 0xCB,
    hl_1: 0x16,
    ix_d_0: 0xDD,
    ix_d_1: 0xCB,
    ix_d_3: 0x16,
    iy_d_0: 0xFD,
    iy_d_1: 0xCB,
    iy_d_3: 0x16,
};

const RRC_CODES: RotShiftGrCodes = RotShiftGrCodes {
    r_0: 0xCB,
    r_1: 0b00001000,
    hl_0: 0xCB,
    hl_1: 0x0E,
    ix_d_0: 0xDD,
    ix_d_1: 0xCB,
    ix_d_3: 0x0E,
    iy_d_0: 0xFD,
    iy_d_1: 0xCB,
    iy_d_3: 0x0E,
};

const RR_CODES: RotShiftGrCodes = RotShiftGrCodes {
    r_0: 0xCB,
    r_1: 0b00011000,
    hl_0: 0xCB,
    hl_1: 0x1E,
    ix_d_0: 0xDD,
    ix_d_1: 0xCB,
    ix_d_3: 0x1E,
    iy_d_0: 0xFD,
    iy_d_1: 0xCB,
    iy_d_3: 0x1E,
};

const SLA_CODES: RotShiftGrCodes = RotShiftGrCodes {
    r_0: 0xCB,
    r_1: 0b00100000,
    hl_0: 0xCB,
    hl_1: 0x26,
    ix_d_0: 0xDD,
    ix_d_1: 0xCB,
    ix_d_3: 0x26,
    iy_d_0: 0xFD,
    iy_d_1: 0xCB,
    iy_d_3: 0x26,
};

const SRA_CODES: RotShiftGrCodes = RotShiftGrCodes {
    r_0: 0xCB,
    r_1: 0b00101000,
    hl_0: 0xCB,
    hl_1: 0x2E,
    ix_d_0: 0xDD,
    ix_d_1: 0xCB,
    ix_d_3: 0x2E,
    iy_d_0: 0xFD,
    iy_d_1: 0xCB,
    iy_d_3: 0x2E,
};

const SRL_CODES: RotShiftGrCodes = RotShiftGrCodes {
    r_0: 0xCB,
    r_1: 0b00111000,
    hl_0: 0xCB,
    hl_1: 0x3E,
    ix_d_0: 0xDD,
    ix_d_1: 0xCB,
    ix_d_3: 0x3E,
    iy_d_0: 0xFD,
    iy_d_1: 0xCB,
    iy_d_3: 0x3E,
};
