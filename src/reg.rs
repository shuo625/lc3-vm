pub const REG_NUM: usize = 10;

pub enum CondFlag {
    POS = 1,
    ZRO = 0,
    NEG = -1,
}

pub enum Reg {
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    PC,
    COND,
}
