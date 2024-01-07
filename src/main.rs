use asm_compiler::compile;
use std::fs::read_to_string;
use virtual_machine::{run, run_debug};

/// Change to true to see the state of the CPU after each instruction is executed.
///
/// Key:
/// * I = instruction pointer
/// * RFA = (general) register, flag (shows 'F' when true), address register
/// * SF = stack pointer, frame pointer
/// * frame = lowest 16 bytes of the current stack frame
const DEBUG: bool = false;

/// Change to run a different program in `mod programs`.
const PATH: &str = programs::RP_3_COUNTER;

/// Add new programs into `src/programs/` and link to them here.
#[allow(unused)]
mod programs {
    pub const RP_1_ADD_NUMBERS: &str = "src/programs/rp_1.txt";
    pub const RP_2_FUNCTION_PARAMETERS: &str = "src/programs/rp_2.txt";
    pub const RP_3_COUNTER: &str = "src/programs/rp_3.txt";

    pub const CAL_RET: &str = "src/programs/cal_ret.txt";
    pub const FACTORIAL: &str = "src/programs/factorial.txt";
    pub const INVALID: &str = "src/programs/invalid.txt";
    pub const PARAMETERS: &str = "src/programs/parameters.txt";
    pub const POWERS_OF_2: &str = "src/programs/powers_of_2.txt";
    pub const STACK_UNDERFLOW: &str = "src/programs/stack_underflow.txt";
}

fn main() {
    let code = read_to_string(PATH).expect(&format!("file not found: {PATH}"));
    let compiled = compile(&code);
    match compiled {
        Ok(bytecode) => {
            if DEBUG {
                run_debug(&bytecode)
            } else {
                run(&bytecode)
            }
        }

        Err(error) => println!("Cannot compile {}: {:?}", PATH, error),
    }
}
