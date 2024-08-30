use crate::program::{Program, ProgramInputs};

pub mod opcodes;
pub use opcodes::OpCode;

mod stack;
use stack::Stack;

mod errors;
use errors::StackError;

#[cfg(test)]
mod tests;

pub struct Processor {
    stack: Stack,
}

impl Processor {
    fn new(inputs: ProgramInputs) -> Processor {
        Processor {
            stack: Stack::new(&inputs),
        }
    }

    pub fn run(program: Program, inputs: ProgramInputs) -> Result<Processor, StackError> {
        let mut processor = Processor::new(inputs);

        for op_code in program.get_code().iter() {
            processor.execute_op(*op_code)?;
        }

        Ok(processor)
    }

    pub fn get_output(&self) -> u128 {
        self.stack.get_stack_top()
    }

    fn execute_op(&mut self, op_code: OpCode) -> Result<(), StackError> {
        self.stack.execute(op_code)
    }
}

impl std::fmt::Debug for Processor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "output is {}", self.get_output())
    }
}
