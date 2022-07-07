use crate::{bits16::Bits16, memory::Memory, reg::Reg};

pub struct LDI {
    dr: usize,
    pc_offset: u16,
}

impl LDI {
    pub fn new(instruction: Bits16) -> Self {
        LDI {
            dr: instruction.bits(9, 11).unwrap() as usize,
            pc_offset: Bits16::sign_entend(instruction.bits(0, 8).unwrap(), 9),
        }
    }

    pub fn exec(&self, regs: &mut Reg, memory: &Memory) {
        regs.Rx[self.dr] = memory.read(memory.read(regs.PC + self.pc_offset));
        regs.update_flag(regs.Rx[self.dr]);
    }
}
