use crate::{
    bits16::Bits16,
    reg::{CondFlag, Reg},
};

pub struct BR {
    n: bool,
    z: bool,
    p: bool,
    pcoffset: u16,
}

impl BR {
    pub fn new(instruction: Bits16) -> Self {
        BR {
            n: if instruction.bit(11).unwrap() == 1 {
                true
            } else {
                false
            },
            z: if instruction.bit(10).unwrap() == 1 {
                true
            } else {
                false
            },
            p: if instruction.bit(9).unwrap() == 1 {
                true
            } else {
                false
            },
            pcoffset: Bits16::sign_entend(instruction.bits(0, 8).unwrap(), 9),
        }
    }

    pub fn exec(&self, regs: &mut Reg) {
        let test = match regs.COND {
            CondFlag::N => self.n || !self.p && !self.z,
            CondFlag::P => self.p || !self.n && !self.z,
            CondFlag::Z => self.z || !self.p && !self.n,
        };

        if test {
            regs.PC += self.pcoffset;
        }
    }
}
