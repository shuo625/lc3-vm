use std::fs::File;

use crate::{instructions::Instruction, memory::Memory, reg::Reg};

pub enum VMErr {
    INVALIDOP,
}

pub struct VM {
    regs: Reg,
    memory: Memory,
}

impl VM {
    pub fn new() -> Self {
        VM {
            regs: Reg::new(),
            memory: Memory::new(),
        }
    }

    pub fn load(&mut self, file: File) {}

    pub fn exec(&mut self) -> Result<(), VMErr> {
        loop {
            let instruction_code = self.memory.read(self.regs.PC as usize);
            match Instruction::new(instruction_code) {
                Some(instruction) => instruction.exec(&mut self.regs),
                None => return Err(VMErr::INVALIDOP),
            }
        }
    }
}
