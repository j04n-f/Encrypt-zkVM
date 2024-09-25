use crate::program::{Program, ProgramInputs};

pub mod opcodes;
pub use opcodes::{OpCode, OpValue, Operation};

mod stack;
use stack::Stack;

mod decoder;
use decoder::Decoder;

mod system;
use system::System;

mod errors;

pub use errors::StackError;

use rand::Rng;

#[cfg(test)]
mod tests;

// Winterfell Constrains
// TraceLength > 7
// TraceLength % 2 = 0
const MIN_TRACE_LENGTH: usize = 8;

pub struct Processor {
    stack: Stack,
    decoder: Decoder,
    system: System,
}

impl Processor {
    fn new(inputs: ProgramInputs) -> Processor {
        Processor {
            stack: Stack::new(&inputs, MIN_TRACE_LENGTH),
            decoder: Decoder::new(MIN_TRACE_LENGTH),
            system: System::new(MIN_TRACE_LENGTH),
        }
    }

    pub fn run(program: Program, inputs: ProgramInputs) -> Result<Processor, StackError> {
        let mut processor = Processor::new(inputs);

        for op in program.get_code().iter() {
            processor.execute_op(op)?;
        }

        Ok(processor)
    }

    pub fn trace(self) -> Vec<Vec<u128>> {
        let mut trace = Vec::new();

        trace.extend(self.system.into_trace());
        trace.extend(self.decoder.into_trace());
        trace.extend(self.stack.into_trace());

        let mut rng = rand::thread_rng();

        for column in &mut trace {
            let last = column.last_mut().unwrap();
            *last = rng.gen();
        }

        trace
    }

    pub fn get_output(&self) -> Vec<u128> {
        self.stack.current_state()
    }

    fn execute_op(&mut self, op: &Operation) -> Result<(), StackError> {
        self.system.advance_step();
        self.stack.execute_op(op)?;
        self.decoder.decode_op(op);
        Ok(())
    }
}

impl std::fmt::Debug for Processor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "output is {:?}", self.get_output())
    }
}
