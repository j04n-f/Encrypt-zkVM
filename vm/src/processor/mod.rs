use crate::program::{Program, ProgramInputs};

pub mod opcodes;
pub use opcodes::{HashOperation, OpCode, Operation};

mod stack;
use stack::Stack;

mod decoder;
use decoder::Decoder;

mod system;
use system::System;

mod chiplets;
use chiplets::Chiplets;

mod errors;

pub use errors::ProcessorError;

use rand::Rng;

use winterfell::math::{fields::f128::BaseElement, FieldElement};

#[cfg(test)]
mod tests;

const ZERO: BaseElement = BaseElement::ZERO;
const ONE: BaseElement = BaseElement::ONE;

// constrains:
// winterfell trace length must be at least 8 and multiple of 2
// rescue-prime hash sponge requries at least 16 rounds
const MIN_TRACE_LENGTH: usize = 16;
const MAX_STACK_DEPTH: usize = 16;

// overwrite last trace row with random values
// winterfell uses trace.length() - 1 to compute the column degree
// it fails to compute the degree when all values are 0
// add a random value to the last row and allow 2 transition exemptions
const NUM_RAND_ROWS: usize = 1;

pub struct Processor {
    stack: Stack,
    decoder: Decoder,
    system: System,
    chiplets: Chiplets,
}

impl Processor {
    fn new(inputs: ProgramInputs) -> Processor {
        Processor {
            stack: Stack::new(&inputs, MIN_TRACE_LENGTH),
            decoder: Decoder::new(MIN_TRACE_LENGTH),
            system: System::new(MIN_TRACE_LENGTH),
            chiplets: Chiplets::new(MIN_TRACE_LENGTH),
        }
    }

    pub fn run(program: Program, inputs: ProgramInputs) -> Result<Processor, ProcessorError> {
        let mut processor = Processor::new(inputs);

        for op in program.get_code().iter() {
            processor.execute_op(op)?;
        }

        Ok(processor)
    }

    pub fn trace(self) -> Vec<Vec<BaseElement>> {
        let mut trace = Vec::new();

        let trace_length = (self.chiplets.trace_length() + NUM_RAND_ROWS).next_power_of_two();

        trace.extend(self.system.into_trace(trace_length));
        trace.extend(self.decoder.into_trace(trace_length));
        trace.extend(self.chiplets.into_trace(trace_length));
        trace.extend(self.stack.into_trace(trace_length));

        let mut rng = rand::thread_rng();

        for column in &mut trace {
            let last = column.last_mut().unwrap();
            // exclude 0 t0 force columns to have at least on value different to 0
            *last = BaseElement::try_from(rng.gen_range(1..=u128::MAX)).unwrap();
        }

        trace
    }

    pub fn get_stack_output(&self) -> Vec<BaseElement> {
        // trace computation does not change the clock value
        // clock value is always set to the last stack row
        self.stack.current_state()
    }

    fn execute_op(&mut self, op: &Operation) -> Result<(), ProcessorError> {
        self.system.advance_step();

        if let Err(err) = self.stack.execute_op(op) {
            return Err(ProcessorError::Stack(err));
        };

        self.decoder.decode_op(op);

        if let Err(err) = self.chiplets.hash_op(op) {
            return Err(ProcessorError::Chiplets(err));
        };

        Ok(())
    }
}

impl std::fmt::Debug for Processor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "output is {:?}", self.get_stack_output())
    }
}
