use std::fs::File;

use crate::{
    instructions::Instruction,
    memory::Memory,
    reg::{Reg, REG_NUM},
};

pub enum VMErr {
    INVALIDOP,
}

pub struct VM {
    regs: [u16; REG_NUM],
    memory: Memory,
}

impl VM {
    pub fn new() -> Self {
        VM {
            regs: [0; REG_NUM],
            memory: Memory::new(),
        }
    }

    pub fn load(&mut self, file: File) {}

    pub fn exec(&mut self) -> Result<(), VMErr> {
        loop {
            let instruction_code = self.memory.read(self.regs[Reg::PC]);
            match Instruction::new(instruction_code) {
                Some(instruction) => instruction.exec(&mut self.regs),
                None => return Err(VMErr::INVALIDOP),
            }
        }
    }
}
