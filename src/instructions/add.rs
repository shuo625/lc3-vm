use crate::bits16::Bits16;

enum AddMode {
    RegisterMode,
    ImmediateMode,
}

pub struct ADD {
    dr: usize,
    sr1: usize,
    sr2: usize,
    imm: u16,
    mode: AddMode,
}

impl ADD {
    pub fn new(instruction: Bits16) -> Self {
        ADD {
            dr: instruction.bits(9, 11).unwrap() as usize,
            sr1: instruction.bits(6, 8).unwrap() as usize,
            sr2: instruction.bits(0, 2).unwrap() as usize,
            imm: instruction.bits(0, 4).unwrap(),
            mode: if instruction.bit(5).unwrap() == 0 {
                AddMode::RegisterMode
            } else {
                AddMode::ImmediateMode
            },
        }
    }

    pub fn exec(&self, regs: &mut [u16]) {
        match self.mode {
            AddMode::ImmediateMode => regs[self.dr as usize] = regs[self.sr1] + self.imm,
            AddMode::RegisterMode => regs[self.dr] = regs[self.sr1] + regs[self.sr2],
        };
    }
}
