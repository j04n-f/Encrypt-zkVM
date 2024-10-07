use crate::program::{Program, ProgramInputs};

pub mod opcodes;
pub use opcodes::{HashOperation, OpCode, OpValue, Operation};

mod stack;
use stack::Stack;

mod decoder;
use decoder::Decoder;

mod system;
use system::System;

mod chiplets;
use chiplets::Chiplets;

mod errors;

pub use errors::StackError;

use rand::Rng;

use winterfell::math::{fields::f128::BaseElement, FieldElement};

#[cfg(test)]
mod tests;

const ZERO: BaseElement = BaseElement::ZERO;
const ONE: BaseElement = BaseElement::ONE;

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

    pub fn run(program: Program, inputs: ProgramInputs) -> Result<Processor, StackError> {
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
        trace.extend(
            self.stack
                .into_trace(trace_length)
                .iter()
                .map(|trace| {
                    trace
                        .iter()
                        .map(|value| BaseElement::try_from(*value).unwrap())
                        .collect::<Vec<BaseElement>>()
                })
                .collect::<Vec<Vec<BaseElement>>>(),
        );

        let mut rng = rand::thread_rng();

        for column in &mut trace {
            let last = column.last_mut().unwrap();
            // exclude 0 t0 force columns to have at least on value different to 0
            *last = BaseElement::try_from(rng.gen_range(1..=u128::MAX)).unwrap();
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
        self.chiplets.hash_op(op);
        Ok(())
    }
}

impl std::fmt::Debug for Processor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "output is {:?}", self.get_stack_output())
    }
}
