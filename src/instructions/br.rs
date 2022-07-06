use crate::bits16::Bits16;

pub struct BR {
    pcoffset: u16,
}

impl BR {
    pub fn new(instruction: Bits16) -> Self {
        BR {
            pcoffset: instruction.bits(0, 8).unwrap(),
        }
    }

    pub fn exec(&self) {}
}
