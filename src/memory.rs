const MEMORY_MAX: usize = 1 << 16;

pub struct Memory {
    mem: [u16; MEMORY_MAX],
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            mem: [0; MEMORY_MAX],
        }
    }

    pub fn read(&self, address: usize) -> u16 {
        self.mem[address]
    }
}
