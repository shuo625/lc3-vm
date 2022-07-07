use crate::{bits16::Bits16, memory::Memory, reg::Reg};

pub struct LD {
    dr: usize,
    pcoffset: u16,
}

impl LD {
    pub fn new(instruction: Bits16) -> Self {
        LD {
            dr: instruction.bits(9, 11).unwrap() as usize,
            pcoffset: Bits16::sign_entend(instruction.bits(0, 8).unwrap(), 9),
        }
    }

    pub fn exec(&self, regs: &mut Reg, memory: &Memory) {
        regs.Rx[self.dr] = memory.read(regs.PC + self.pcoffset);
        regs.update_flag(regs.Rx[self.dr]);
    }
}
