use crate::domain::enums::{ShortReg, WideReg};

pub enum ParsedRegister {
    WideReg(WideReg),
    ShortReg(ShortReg),
    Error(String),
}

pub fn parse_register(identifier: &str) -> ParsedRegister {
    match identifier.to_lowercase().as_str() {
        "a" => ParsedRegister::ShortReg(ShortReg::A),
        "b" => ParsedRegister::ShortReg(ShortReg::B),
        "c" => ParsedRegister::ShortReg(ShortReg::C),
        "d" => ParsedRegister::ShortReg(ShortReg::D),
        "e" => ParsedRegister::ShortReg(ShortReg::E),
        "h" => ParsedRegister::ShortReg(ShortReg::H),
        "l" => ParsedRegister::ShortReg(ShortReg::L),
        "i" => ParsedRegister::ShortReg(ShortReg::I),
        "r" => ParsedRegister::ShortReg(ShortReg::R),
        "af" => ParsedRegister::WideReg(WideReg::AF),
        "afp" => ParsedRegister::WideReg(WideReg::AFp),
        "bc" => ParsedRegister::WideReg(WideReg::BC),
        "de" => ParsedRegister::WideReg(WideReg::DE),
        "hl" => ParsedRegister::WideReg(WideReg::HL),
        "sp" => ParsedRegister::WideReg(WideReg::SP),
        "ix" => ParsedRegister::WideReg(WideReg::IX),
        "iy" => ParsedRegister::WideReg(WideReg::IY),
        _ => unimplemented!("unimplemented identifier handler: {:?}", identifier),
    }
}
