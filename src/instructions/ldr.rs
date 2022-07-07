use crate::{bits16::Bits16, memory::Memory, reg::Reg};

pub struct LDR {
    dr: usize,
    baser: usize,
    offset: usize,
}

impl LDR {
    pub fn new(instruction: Bits16) -> Self {
        LDR {
            dr: instruction.bits(9, 11).unwrap() as usize,
            baser: instruction.bits(6, 8).unwrap() as usize,
            offset: Bits16::sign_entend(instruction.bits(0, 5).unwrap(), 6) as usize,
        }
    }

    pub fn exec(&self, regs: &mut Reg, memory: &Memory) {
        regs.Rx[self.dr] = memory.read(self.baser + self.offset);
        regs.update_flag(regs.Rx[self.dr]);
    }
}
