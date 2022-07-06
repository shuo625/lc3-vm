pub const GEN_REG_NUM: usize = 10;

pub enum CondFlag {
    POS = 1,
    ZRO = 0,
    NEG = -1,
}

pub struct Reg {
    pub Rx: [u16; GEN_REG_NUM],
    pub PC: u16,
    pub COND: CondFlag,
}

impl Reg {
    pub fn new() -> Self {
        Reg {
            Rx: [0; GEN_REG_NUM],
            PC: 0,
            COND: CondFlag::ZRO,
        }
    }

    pub fn update_flag(&mut self, x: u16) {
        self.COND = if x == 0 {
            CondFlag::ZRO
        } else if (x >> 15) == 1 {
            CondFlag::NEG
        } else {
            CondFlag::POS
        }
    }
}
