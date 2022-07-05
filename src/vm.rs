use std::fs::File;

use crate::{
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

    pub fn exec(&self) -> Result<(), VMErr> {
        todo!()
    }
}
