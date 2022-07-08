pub const GEN_REG_NUM: usize = 8;

pub enum CondFlag {
    P,
    Z,
    N,
}

#[allow(non_snake_case)]
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
            COND: CondFlag::Z,
        }
    }

    pub fn update_flag(&mut self, x: u16) {
        self.COND = if x == 0 {
            CondFlag::Z
        } else if (x >> 15) == 1 {
            CondFlag::N
        } else {
            CondFlag::P
        }
    }
}
