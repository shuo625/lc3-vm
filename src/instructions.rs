mod add;
mod br;
mod ld;

use crate::{bits16::Bits16, reg::Reg};

pub enum Instruction {
    BR(br::BR),    // branch
    ADD(add::ADD), // add
    LD,            // load
    ST,            // store
    JSR,           // jump register
    AND,           // bit and
    LDR,           // load register
    STR,           // store register
    RTI,           // unused
    NOT,           // bit not
    LDI,           // load indirect
    STI,           // store indirect
    JMP,           // jump
    RES,           // reserved unused
    LEA,           // load effective address
    TRAP,          // execute trap
}

impl Instruction {
    pub fn new(instruction_code: u16) -> Option<Self> {
        let instruction = Bits16::new(instruction_code);
        let op_code = instruction.bits(12, 15).unwrap();

        match op_code {
            0 => Some(Instruction::BR(br::BR::new(instruction))),
            1 => Some(Instruction::ADD(add::ADD::new(instruction))),
            _ => None,
        }
    }

    pub fn exec(&self, regs: &mut Reg) {
        match self {
            Instruction::ADD(add) => add.exec(regs),
            _ => {}
        }
    }
}
