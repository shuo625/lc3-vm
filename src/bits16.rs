#[derive(Clone, Copy)]
pub struct Bits16 {
    bits: u16,
}

impl Bits16 {
    pub fn new(bits: u16) -> Self {
        Bits16 { bits }
    }

    pub fn bit(&self, bit_index: usize) -> Option<u16> {
        if bit_index < 16 {
            Some((self.bits >> bit_index) & 0x1)
        } else {
            None
        }
    }

    pub fn bits(&self, bit_start: usize, bit_end: usize) -> Option<u16> {
        if bit_start > 15 || bit_end > 15 || bit_start > bit_end {
            None
        } else {
            Some((self.bits >> bit_start) & (0x1 << (bit_end - bit_start)))
        }
    }

    pub fn sign_entend(x: u16, bit_count: usize) -> u16 {
        if (x >> (bit_count - 1)) & 1 == 1 {
            x | (0xFFFF << bit_count)
        } else {
            x
        }
    }
}
