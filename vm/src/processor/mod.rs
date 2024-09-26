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

// winterfell constrains
// trace length must be at least 8 and multiple of 2
const MIN_TRACE_LENGTH: usize = 8;
const MAX_STACK_DEPTH: usize = 8;

// overwrite last trace row with random values
// winterfell uses trace.length() - 1 to compute the column degree
// it fails to compute the degree when all values are 0
// add a random value to the last row and allow 2 transition exemptions
const NUM_RAND_ROWS: usize = 1;

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

        let trace_length = {
            let traces_len = [
                self.system.trace_length(),
                self.stack.trace_length(),
                self.decoder.trace_length(),
            ];

            let max_len = traces_len.iter().max().unwrap();

            (max_len + NUM_RAND_ROWS).next_power_of_two()
        };

        trace.extend(self.system.into_trace(trace_length));
        trace.extend(self.decoder.into_trace(trace_length));
        trace.extend(self.stack.into_trace(trace_length));

        let mut rng = rand::thread_rng();

        for column in &mut trace {
            let last = column.last_mut().unwrap();
            // exclude 0 t0 force columns to have at least on value different to 0
            *last = rng.gen_range(1..=u128::MAX);
        }

        trace
    }

    pub fn get_stack_output(&self) -> Vec<u128> {
        // trace computation does not change the clock value
        // clock value is always set to the last stack row
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
        write!(f, "output is {:?}", self.get_stack_output())
    }
}
