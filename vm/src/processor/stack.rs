use super::errors::StackError;
use super::ProgramInputs;
use super::{OpCode, Operation, ZERO};

use fhe::{FheUInt8, ServerKey};

use winterfell::math::fields::f128::BaseElement;

use std::ops::{Add, Mul};

use super::MAX_STACK_DEPTH;

pub struct Stack {
    clk: usize,
    registers: Vec<Vec<BaseElement>>,
    helpers: Vec<Vec<BaseElement>>,
    tape_a: Vec<u8>,
    tape_b: Vec<FheUInt8>,
    depth: usize,
    server_key: ServerKey,
    trace_length: usize,
}

impl Stack {
    pub fn new(inputs: &ProgramInputs, init_trace_length: usize) -> Stack {
        let registers: Vec<Vec<BaseElement>> = (0..MAX_STACK_DEPTH).map(|_| vec![ZERO; init_trace_length]).collect();

        let helpers: Vec<Vec<BaseElement>> = (0..1).map(|_| vec![ZERO; init_trace_length]).collect();

        // reverse inputs to pop them in order
        let mut tape_a = inputs.get_public().to_vec();
        tape_a.reverse();
        let mut tape_b = inputs.get_secret().to_vec();
        tape_b.reverse();

        Stack {
            clk: 0,
            registers,
            helpers,
            tape_a,
            tape_b,
            depth: 0,
            server_key: inputs.get_server_key(),
            trace_length: init_trace_length,
        }
    }

    pub fn execute_op(&mut self, op: &Operation) -> Result<(), StackError> {
        self.advance_clock();
        self.ensure_trace_capacity();

        #[rustfmt::skip]
        match op.op_code() {
            OpCode::Noop    => self.op_noop(),

            OpCode::Push    => self.op_push(op),
            OpCode::Read    => self.op_read(op),
            OpCode::Read2   => self.op_read2(op),

            OpCode::Add     => self.op_add(op),
            OpCode::Mul     => self.op_mul(op),
            OpCode::SAdd    => self.op_sadd(op),
            OpCode::SMul    => self.op_smul(op),
            OpCode::Add2    => self.op_add2(op),
        }?;

        self.set_helpers();

        Ok(())
    }

    pub fn current_stack_state(&self) -> [BaseElement; MAX_STACK_DEPTH] {
        let mut state = Vec::with_capacity(MAX_STACK_DEPTH);
        for i in 0..MAX_STACK_DEPTH {
            state.push(self.registers[i][self.clk]);
        }
        state.try_into().unwrap()
    }

    pub fn into_trace(mut self, trace_length: usize) -> Vec<Vec<BaseElement>> {
        let mut trace = Vec::new();

        for col in self.registers.iter_mut() {
            col.resize(self.clk + 1, ZERO);
            col.resize(trace_length, col[self.clk]);
        }

        for col in self.helpers.iter_mut() {
            col.resize(self.clk + 1, ZERO);
            col.resize(trace_length, col[self.clk]);
        }

        trace.append(&mut self.helpers);
        trace.append(&mut self.registers);

        trace
    }

    fn op_noop(&mut self) -> Result<(), StackError> {
        for i in 0..self.depth {
            self.registers[i][self.clk] = self.registers[i][self.clk - 1];
        }
        Ok(())
    }

    fn op_push(&mut self, op: &Operation) -> Result<(), StackError> {
        self.shift_right(op, 0, 1)?;
        self.registers[0][self.clk] = BaseElement::from(op.value());
        Ok(())
    }

    fn op_read(&mut self, op: &Operation) -> Result<(), StackError> {
        self.shift_right(op, 0, 1)?;
        let value = match self.tape_a.pop() {
            Some(value) => value,
            None => return Err(StackError::empty_inputs(op, self.clk)),
        };
        self.registers[0][self.clk] = BaseElement::from(value);
        Ok(())
    }

    fn op_read2(&mut self, op: &Operation) -> Result<(), StackError> {
        let ct = match self.tape_b.pop() {
            Some(value) => value.ciphertext().to_vec(),
            None => return Err(StackError::empty_inputs(op, self.clk)),
        };
        self.shift_right(op, 0, ct.len())?;
        for (i, value) in ct.iter().enumerate() {
            self.registers[i][self.clk] = *value;
        }
        Ok(())
    }

    fn op_add(&mut self, op: &Operation) -> Result<(), StackError> {
        if self.depth < 2 {
            return Err(StackError::stack_underflow(op, self.clk));
        }

        let x = self.registers[0][self.clk - 1];
        let y = self.registers[1][self.clk - 1];
        self.registers[0][self.clk] = x.add(y);
        self.shift_left(op, 2, 1)
    }

    fn op_mul(&mut self, op: &Operation) -> Result<(), StackError> {
        if self.depth < 2 {
            return Err(StackError::stack_underflow(op, self.clk));
        }
        let x = self.registers[0][self.clk - 1];
        let y = self.registers[1][self.clk - 1];
        self.registers[0][self.clk] = x.mul(y);
        self.shift_left(op, 2, 1)
    }

    fn op_sadd(&mut self, op: &Operation) -> Result<(), StackError> {
        let lwe_size = self.server_key.lwe_size();

        if self.depth < lwe_size + 1 {
            return Err(StackError::stack_underflow(op, self.clk));
        }

        let ct: Vec<BaseElement> = (1..(lwe_size + 1))
            .map(|i: usize| self.registers[i][self.clk - 1])
            .collect();

        let scalar = self.registers[0][self.clk - 1];

        let result_ct = self.server_key.scalar_add(&scalar, &FheUInt8::new(&ct));

        for (i, value) in result_ct.ciphertext().iter().enumerate() {
            self.registers[i][self.clk] = *value;
        }

        self.shift_left(op, lwe_size + 1, 1)
    }

    fn op_smul(&mut self, op: &Operation) -> Result<(), StackError> {
        let lwe_size = self.server_key.lwe_size();

        if self.depth < lwe_size + 1 {
            return Err(StackError::stack_underflow(op, self.clk));
        }

        let ct: Vec<BaseElement> = (1..(lwe_size + 1))
            .map(|i: usize| self.registers[i][self.clk - 1])
            .collect();

        let scalar = self.registers[0][self.clk - 1];

        let result_ct = self.server_key.scalar_mul(&scalar, &FheUInt8::new(&ct));

        for (i, value) in result_ct.ciphertext().iter().enumerate() {
            self.registers[i][self.clk] = *value;
        }

        self.shift_left(op, lwe_size + 1, 1)
    }

    fn op_add2(&mut self, op: &Operation) -> Result<(), StackError> {
        let lwe_size = self.server_key.lwe_size();

        if self.depth < lwe_size * 2 {
            return Err(StackError::stack_underflow(op, self.clk));
        }

        let ct0: Vec<BaseElement> = (0..lwe_size).map(|i: usize| self.registers[i][self.clk - 1]).collect();
        let ct1: Vec<BaseElement> = (0..lwe_size)
            .map(|i: usize| self.registers[i + lwe_size][self.clk - 1])
            .collect();

        let result_ct = self.server_key.add(&FheUInt8::new(&ct0), &FheUInt8::new(&ct1));

        for (i, value) in result_ct.ciphertext().iter().enumerate() {
            self.registers[i][self.clk] = *value;
        }

        self.shift_left(op, lwe_size * 2, lwe_size)
    }

    fn shift_left(&mut self, op: &Operation, start: usize, pos_count: usize) -> Result<(), StackError> {
        if self.depth < pos_count {
            return Err(StackError::stack_underflow(op, self.clk));
        }

        // shift all values by pos_count to the left
        for i in start..self.depth {
            self.registers[i - pos_count][self.clk] = self.registers[i][self.clk - 1];
        }

        // set all "shifted-in" slots to 0
        for i in (self.depth - pos_count)..self.depth {
            self.registers[i][self.clk] = ZERO;
        }

        // stack depth has been reduced by pos_count
        self.depth -= pos_count;

        Ok(())
    }

    fn shift_right(&mut self, op: &Operation, start: usize, pos_count: usize) -> Result<(), StackError> {
        // stack depth has been increased by pos_count
        self.depth += pos_count;

        // allocate new registers to the stack and increase the stack mac depth
        if self.depth > MAX_STACK_DEPTH {
            return Err(StackError::stack_overflow(op, self.clk));
        }

        // set all "shifted-in" slots to clk' - 1
        for i in start..(self.depth - pos_count) {
            self.registers[i + pos_count][self.clk] = self.registers[i][self.clk - 1];
        }

        Ok(())
    }

    // Ensure there is enough memory allocated for the trace to accommodate a new row.
    // Trace length is doubled every time it needs to be increased.
    // Constrain: trace_length % 2 = 0.
    fn ensure_trace_capacity(&mut self) {
        if self.clk >= self.trace_length {
            self.trace_length *= 2;
            for col in self.registers.iter_mut() {
                col.resize(self.trace_length, ZERO);
            }
            for col in self.helpers.iter_mut() {
                col.resize(self.trace_length, ZERO);
            }
        }
    }

    // Increment clock by 1
    fn advance_clock(&mut self) {
        self.clk += 1;
    }

    fn set_helpers(&mut self) {
        self.helpers[0][self.clk] = BaseElement::from(self.depth as u32);
    }
}
