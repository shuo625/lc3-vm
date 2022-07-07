use crate::{bits16::Bits16, reg::Reg};

pub struct LEA {
    dr: usize,
    pcoffset: u16,
}

impl LEA {
    pub fn new(instruction: Bits16) -> Self {
        LEA {
            dr: instruction.bits(9, 11).unwrap() as usize,
            pcoffset: Bits16::sign_entend(instruction.bits(0, 8).unwrap(), 9),
        }
    }

    pub fn exec(&self, regs: &mut Reg) {
        regs.Rx[self.dr] = regs.PC + self.pcoffset;
        regs.update_flag(regs.Rx[self.dr]);
    }
}
