use std::env;

use lc3_vm::{VMErr, VM};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let mut vm = VM::new(&filename);

    match vm.exec() {
        Ok(_) => {}
        Err(err) => match err {
            VMErr::INVALIDOP => println!("there are invalid op in the program"),
        },
    }
}
