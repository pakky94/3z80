#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ShortReg {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum WideReg {
    BC,
    DE,
    HL,
    SP,
    IX,
    IY,
}
