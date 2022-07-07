use crate::{bits16::Bits16, memory::Memory, reg::Reg};

pub struct ST {
    sr: usize,
    pcoffset: u16,
}

impl ST {
    pub fn new(instruction: Bits16) -> Self {
        ST {
            sr: instruction.bits(9, 11).unwrap() as usize,
            pcoffset: Bits16::sign_entend(instruction.bits(0, 8).unwrap(), 9),
        }
    }

    pub fn exec(&self, regs: &Reg, memory: &mut Memory) {
        memory.write(regs.PC + self.pcoffset, regs.Rx[self.sr]);
    }
}
