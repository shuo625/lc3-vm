mod add;
mod and;
mod br;
mod jmp;
mod jsr;
mod ld;
mod ldi;
mod ldr;
mod lea;
mod not;
mod st;
mod sti;
mod str;
mod trap;

use crate::{bits16::Bits16, memory::Memory, reg::Reg};

const OP_BR: u16 = 0b0000;
const OP_ADD: u16 = 0b0001;
const OP_LD: u16 = 0b0010;
const OP_ST: u16 = 0b0011;
const OP_JSR: u16 = 0b0100;
const OP_AND: u16 = 0b0101;
const OP_LDR: u16 = 0b0110;
const OP_STR: u16 = 0b0111;
#[allow(unused)]
const OP_RTI: u16 = 0b1000;
const OP_NOT: u16 = 0b1001;
const OP_LDI: u16 = 0b1010;
const OP_STI: u16 = 0b1011;
const OP_JMP: u16 = 0b1100;
#[allow(unused)]
const OP_RES: u16 = 0b1101;
const OP_LEA: u16 = 0b1110;
const OP_TRAP: u16 = 0b1111;

pub enum Instruction {
    BR(br::BR),    // branch
    ADD(add::ADD), // add
    LD(ld::LD),    // load
    ST(st::ST),    // store
    JSR(jsr::JSR), // jump register
    AND(and::AND), // bit and
    LDR(ldr::LDR), // load register
    STR(str::STR), // store register
    #[allow(dead_code)]
    RTI, // unused
    NOT(not::NOT), // bit not
    LDI(ldi::LDI), // load indirect
    STI(sti::STI), // store indirect
    JMP(jmp::JMP), // call and ret
    #[allow(dead_code)]
    RES, // reserved unused
    LEA(lea::LEA), // load effective address
    TRAP(trap::TRAP), // execute trap
}

impl Instruction {
    pub fn new(instruction_code: u16) -> Option<Self> {
        let instruction = Bits16::new(instruction_code);
        let op_code = instruction.bits(12, 15).unwrap();

        match op_code {
            OP_BR => Some(Instruction::BR(br::BR::new(instruction))),
            OP_ADD => Some(Instruction::ADD(add::ADD::new(instruction))),
            OP_LD => Some(Instruction::LD(ld::LD::new(instruction))),
            OP_ST => Some(Instruction::ST(st::ST::new(instruction))),
            OP_JSR => Some(Instruction::JSR(jsr::JSR::new(instruction))),
            OP_AND => Some(Instruction::AND(and::AND::new(instruction))),
            OP_LDR => Some(Instruction::LDR(ldr::LDR::new(instruction))),
            OP_STR => Some(Instruction::STR(str::STR::new(instruction))),
            OP_NOT => Some(Instruction::NOT(not::NOT::new(instruction))),
            OP_LDI => Some(Instruction::LDI(ldi::LDI::new(instruction))),
            OP_STI => Some(Instruction::STI(sti::STI::new(instruction))),
            OP_JMP => Some(Instruction::JMP(jmp::JMP::new(instruction))),
            OP_LEA => Some(Instruction::LEA(lea::LEA::new(instruction))),
            OP_TRAP => Some(Instruction::TRAP(trap::TRAP::new(instruction))),
            _ => None,
        }
    }

    pub fn exec(&self, regs: &mut Reg, memory: &mut Memory) {
        match self {
            Instruction::BR(br) => br.exec(regs),
            Instruction::ADD(add) => add.exec(regs),
            Instruction::LD(ld) => ld.exec(regs, memory),
            Instruction::ST(st) => st.exec(regs, memory),
            Instruction::JSR(jsr) => jsr.exec(regs),
            Instruction::AND(and) => and.exec(regs),
            Instruction::LDR(ldr) => ldr.exec(regs, memory),
            Instruction::STR(str) => str.exec(regs, memory),
            Instruction::NOT(not) => not.exec(regs),
            Instruction::LDI(ldi) => ldi.exec(regs, memory),
            Instruction::STI(sti) => sti.exec(regs, memory),
            Instruction::JMP(jmp) => jmp.exec(regs),
            Instruction::LEA(lea) => lea.exec(regs),
            Instruction::TRAP(trap) => trap.exec(regs, memory),
            _ => {}
        }
    }
}
