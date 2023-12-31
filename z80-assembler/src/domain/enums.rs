#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ShortReg {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    I,
    R,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum WideReg {
    AF,
    AFp,
    BC,
    DE,
    HL,
    SP,
    IX,
    IY,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Condition {
    NZ,
    Z,
    NC,
    C,
    PO,
    PE,
    P,
    M,
}
