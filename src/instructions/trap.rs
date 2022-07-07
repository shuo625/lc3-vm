use std::io::Read;

use crate::{bits16::Bits16, memory::Memory, reg::Reg};

const TRAP_GETC: u16 = 0x20;
const TRAP_OUT: u16 = 0x21;
const TRAP_PUTS: u16 = 0x22;
const TRAP_IN: u16 = 0x23;
const TRAP_PUTSP: u16 = 0x24;
const TRAP_HALT: u16 = 0x25;

const STRING_END: u16 = 0x0000;

pub struct TRAP {
    trapvec: u16,
}

impl TRAP {
    pub fn new(instruction: Bits16) -> Self {
        TRAP {
            trapvec: instruction.bits(0, 7).unwrap(),
        }
    }

    pub fn exec(&self, regs: &mut Reg, memory: &Memory) {
        match self.trapvec {
            TRAP_GETC => TRAP::trap_getc(regs),
            TRAP_OUT => TRAP::trap_out(regs),
            TRAP_PUTS => TRAP::trap_puts(regs, memory),
            TRAP_IN => TRAP::trap_in(regs),
            TRAP_PUTSP => TRAP::trap_putsp(regs, memory),
            TRAP_HALT => TRAP::trap_halt(),
            _ => panic!("Invalid trap code"),
        }
    }

    fn trap_getc(regs: &mut Reg) {
        let c = std::io::stdin().bytes().next().unwrap().unwrap();
        regs.Rx[0] = c as u16;
        regs.update_flag(regs.Rx[0]);
    }

    fn trap_out(regs: &Reg) {
        println!("{}", regs.Rx[0] as u8 as char);
    }

    fn trap_puts(regs: &Reg, memory: &Memory) {
        let mut str_address = regs.Rx[0];
        let mut str_out = String::new();

        loop {
            let str_u16 = memory.read(str_address);
            if str_u16 == STRING_END {
                break;
            }
            str_out.push(str_u16 as u8 as char);
            str_address += 1;
        }

        println!("{}", str_out);
    }

    fn trap_in(regs: &mut Reg) {
        println!("Enter a char:");
        let c = std::io::stdin().bytes().next().unwrap().unwrap();
        println!("{}", c);
        regs.Rx[0] = c as u16;
        regs.update_flag(regs.Rx[0]);
    }

    fn trap_putsp(regs: &Reg, memory: &Memory) {
        let mut str_address = regs.Rx[0];
        let mut str_out = String::new();

        loop {
            let str_u16 = memory.read(str_address);
            if str_u16 == STRING_END {
                break;
            }
            str_out.push(str_u16 as u8 as char);
            str_out.push((str_u16 >> 8) as u8 as char);
            str_address += 1;
        }

        println!("{}", str_out);
    }

    fn trap_halt() {
        panic!("does not support halt");
    }
}
