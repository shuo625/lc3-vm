use crate::{bits16::Bits16, reg::Reg};

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
            imm: Bits16::sign_entend(instruction.bits(0, 4).unwrap(), 5),
            mode: if instruction.bit(5).unwrap() == 0 {
                AddMode::RegisterMode
            } else {
                AddMode::ImmediateMode
            },
        }
    }

    pub fn exec(&self, regs: &mut Reg) {
        match self.mode {
            AddMode::ImmediateMode => regs.Rx[self.dr] = regs.Rx[self.sr1] + self.imm,
            AddMode::RegisterMode => regs.Rx[self.dr] = regs.Rx[self.sr1] + regs.Rx[self.sr2],
        };

        regs.update_flag(regs.Rx[self.dr]);
    }
}
