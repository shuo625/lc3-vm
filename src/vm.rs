use std::fs;

use crate::{instructions::Instruction, memory::Memory, reg::Reg};

pub enum VMErr {
    INVALIDOP,
}

pub struct VM {
    regs: Reg,
    memory: Memory,
}

impl VM {
    pub fn new(filename: &str) -> Self {
        let mut vm = VM {
            regs: Reg::new(),
            memory: Memory::new(),
        };
        vm.load(filename);
        vm
    }

    fn load(&mut self, filename: &str) {
        let content = fs::read_to_string(filename).unwrap();
        let bytes = content.as_bytes();
        let mut i: usize = 0;
        let origin = VM::concat_to_u16(bytes[i], bytes[i + 1]);
        let mut mem_address = origin;

        i = 2;
        loop {
            if i >= bytes.len() {
                break;
            }
            self.memory
                .write(mem_address, VM::concat_to_u16(bytes[i], bytes[i + 1]));
            mem_address += 1;
            i += 2;
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
