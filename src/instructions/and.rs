use crate::{bits16::Bits16, reg::Reg};

enum AndMode {
    RegMode,
    ImmMode,
}

pub struct AND {
    dr: usize,
    sr1: usize,
    sr2: usize,
    imm: u16,
    mode: AndMode,
}

impl AND {
    pub fn new(instruction: Bits16) -> Self {
        AND {
            dr: instruction.bits(9, 11).unwrap() as usize,
            sr1: instruction.bits(6, 8).unwrap() as usize,
            sr2: instruction.bits(0, 2).unwrap() as usize,
            imm: Bits16::sign_entend(instruction.bits(0, 4).unwrap(), 5),
            mode: if instruction.bit(5).unwrap() == 0 {
                AndMode::RegMode
            } else {
                AndMode::ImmMode
            },
        }
    }

    pub fn exec(&self, regs: &mut Reg) {
        match self.mode {
            AndMode::RegMode => regs.Rx[self.dr] = regs.Rx[self.sr1] & regs.Rx[self.sr2],
            AndMode::ImmMode => regs.Rx[self.dr] = regs.Rx[self.sr1] & self.imm,
        }

        regs.update_flag(regs.Rx[self.dr]);
    }
}
