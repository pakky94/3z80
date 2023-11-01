use crate::domain::enums::Condition;

pub fn parse_condition(identifier: &str) -> Option<Condition> {
    match identifier.to_lowercase().as_str() {
        "nz" => Some(Condition::NZ),
        "z" => Some(Condition::Z),
        "nc" => Some(Condition::NC),
        "c" => Some(Condition::C),
        "po" => Some(Condition::PO),
        "pe" => Some(Condition::PE),
        "p" => Some(Condition::P),
        "m" => Some(Condition::M),
        _ => None,
    }
}

pub fn condition_allowed(instr: &str) -> bool {
    match instr {
        "call" | "jp" | "jr" | "ret" => true,
        _ => false,
    }
}
