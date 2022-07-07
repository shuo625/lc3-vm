use crate::{bits16::Bits16, memory::Memory, reg::Reg};

pub struct STR {
    sr: usize,
    baser: usize,
    offset: u16,
}

impl STR {
    pub fn new(instruction: Bits16) -> Self {
        STR {
            sr: instruction.bits(9, 11).unwrap() as usize,
            baser: instruction.bits(6, 8).unwrap() as usize,
            offset: Bits16::sign_entend(instruction.bits(0, 5).unwrap(), 6),
        }
    }

    pub fn exec(&self, regs: &Reg, memory: &mut Memory) {
        memory.write(regs.Rx[self.baser] + self.offset, regs.Rx[self.sr]);
    }
}
