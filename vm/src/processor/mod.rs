use crate::program::{Program, ProgramInputs};

pub mod opcodes;
pub use opcodes::OpCode;

mod stack;
use stack::Stack;

mod errors;

pub use errors::StackError;

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

    pub fn trace(self) -> Vec<Vec<u128>> {
        self.stack.into_trace()
    }

    pub fn get_output(&self) -> Vec<u128> {
        self.stack.current_state()
    }

    fn execute_op(&mut self, op_code: OpCode) -> Result<(), StackError> {
        self.stack.execute(op_code)
    }
}

impl std::fmt::Debug for Processor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "output is {:?}", self.get_output())
    }
}
