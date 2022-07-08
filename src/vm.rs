use std::{fs::File, io::Read};

use crate::{
    instructions::Instruction,
    memory::{Memory, MEMORY_MAX},
    reg::Reg,
};

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

    pub fn load(&mut self, file: File) {
        let mut bytes = file.bytes();
        let origin = VM::concat_to_u16(
            bytes.next().unwrap().unwrap(),
            bytes.next().unwrap().unwrap(),
        );
        let mut mem_address = origin;

        while (mem_address as usize) < MEMORY_MAX {
            self.memory.write(
                mem_address,
                VM::concat_to_u16(
                    bytes.next().unwrap().unwrap(),
                    bytes.next().unwrap().unwrap(),
                ),
            );
            mem_address += 1;
        }
    }

    pub fn exec(&mut self) -> Result<(), VMErr> {
        loop {
            let instruction_code = self.memory.read(self.regs.PC);
            match Instruction::new(instruction_code) {
                Some(instruction) => instruction.exec(&mut self.regs, &mut self.memory),
                None => return Err(VMErr::INVALIDOP),
            }
        }
    }

    fn concat_to_u16(low: u8, high: u8) -> u16 {
        ((low as u16) << 8) | high as u16
    }
}
