pub enum CondFlag {
    FL_POS,
    FL_ZRO,
    FL_NEG,
}

pub struct Reg {
    R_R0: u16,
    R_R1: u16,
    R_R2: u16,
    R_R3: u16,
    R_R4: u16,
    R_R5: u16,
    R_R6: u16,
    R_R7: u16,
    R_PC: u16,
    R_COND: CondFlag,
}

impl Reg {
    pub fn new() -> Self {
        Reg {
            R_R0: 0,
            R_R1: 0,
            R_R2: 0,
            R_R3: 0,
            R_R4: 0,
            R_R5: 0,
            R_R6: 0,
            R_R7: 0,
            R_PC: 0,
            R_COND: CondFlag::FL_ZRO,
        }
    }
}
