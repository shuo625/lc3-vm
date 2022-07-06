use crate::{bits16::Bits16, reg::Reg};

pub struct JMP {
    baser: usize,
}

impl JMP {
    pub fn new(instruction: Bits16) -> Self {
        JMP {
            baser: instruction.bits(6, 8).unwrap() as usize,
        }
    }

    pub fn exec(&self, regs: &mut Reg) {
        regs.PC = regs.Rx[self.baser];
    }
}
