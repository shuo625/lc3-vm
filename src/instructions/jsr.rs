use crate::{bits16::Bits16, reg::Reg};

enum JsrMode {
    RegMode,
    ImmMode,
}

pub struct JSR {
    pcoffset: u16,
    baser: usize,
    mode: JsrMode,
}

impl JSR {
    pub fn new(instruction: Bits16) -> Self {
        JSR {
            pcoffset: Bits16::sign_entend(instruction.bits(0, 11).unwrap(), 11),
            baser: instruction.bits(6, 8).unwrap() as usize,
            mode: if instruction.bit(11).unwrap() == 1 {
                JsrMode::ImmMode
            } else {
                JsrMode::RegMode
            },
        }
    }

    pub fn exec(&self, regs: &mut Reg) {
        regs.Rx[7] = regs.PC;
        regs.PC = match self.mode {
            JsrMode::ImmMode => self.pcoffset + regs.PC,
            JsrMode::RegMode => regs.Rx[self.baser],
        }
    }
}
