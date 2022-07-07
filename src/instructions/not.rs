use crate::{bits16::Bits16, reg::Reg};

pub struct NOT {
    dr: usize,
    sr: usize,
}

impl NOT {
    pub fn new(instruction: Bits16) -> Self {
        NOT {
            dr: instruction.bits(9, 11).unwrap() as usize,
            sr: instruction.bits(6, 8).unwrap() as usize,
        }
    }

    pub fn exec(&self, regs: &mut Reg) {
        regs.Rx[self.dr] = !regs.Rx[self.sr];
        regs.update_flag(regs.Rx[self.dr]);
    }
}
