use super::OpCode;
use super::ProgramInputs;
use super::StackError;

use fhe::{FheUInt8, ServerKey};

const MIN_STACK_DEPTH: usize = 4;

// Winterfell Constrains
// TraceLength > 7
// TraceLength % 2 = 0
const MIN_TRACE_LENGTH: usize = 8;

pub struct Stack {
    registers: Vec<Vec<u128>>,
    clk_trace: Vec<u128>,
    tape_a: Vec<u8>,
    tape_b: Vec<FheUInt8>,
    max_depth: usize,
    depth: usize,
    clk: usize,
    server_key: ServerKey,
}

impl Stack {
    pub fn new(inputs: &ProgramInputs) -> Stack {
        let registers: Vec<Vec<u128>> = (0..MIN_STACK_DEPTH)
            .map(|_| vec![0; MIN_TRACE_LENGTH])
            .collect();
        let clk_trace: Vec<u128> = (0..MIN_TRACE_LENGTH as u128).collect();

        // reverse inputs to pop them in order
        let mut tape_a = inputs.get_public();
        tape_a.reverse();
        let mut tape_b = inputs.get_secret();
        tape_b.reverse();

        Stack {
            registers,
            clk_trace,
            tape_a,
            tape_b,
            max_depth: 0,
            depth: 0,
            clk: 0,
            server_key: inputs.get_server_key(),
        }
    }

    pub fn execute(&mut self, op_code: OpCode) -> Result<(), StackError> {
        self.advance_clock();
        self.ensure_trace_capacity();

        #[rustfmt::skip]
        return match op_code {
            OpCode::Push(value) => self.op_push(value),
            OpCode::Read              => self.op_read(),
            OpCode::Read2             => self.op_read2(),

            OpCode::Add               => self.op_add(op_code),
            OpCode::SAdd              => self.op_sadd(op_code),
            OpCode::Mul               => self.op_mul(op_code),
            OpCode::SMul              => self.op_smul(op_code),
        };
    }

    pub fn trace_length(&self) -> usize {
        self.registers[0].len()
    }

    #[cfg(test)]
    pub fn stack_state(&self, clk: usize) -> Vec<u128> {
        let mut state = Vec::with_capacity(self.registers.len());
        for i in 0..self.registers.len() {
            state.push(self.registers[i][clk]);
        }
        state
    }

    pub fn current_state(&self) -> Vec<u128> {
        let mut state = Vec::with_capacity(self.registers.len());
        for i in 0..self.registers.len() {
            state.push(self.registers[i][self.clk]);
        }
        state
    }

    pub fn into_trace(mut self) -> Vec<Vec<u128>> {
        let trace_length = self.trace_length();

        for register in self.registers.iter_mut() {
            register.resize(self.clk + 1, 0);
            register.resize(trace_length, register[self.clk]);
        }

        self.registers.truncate(self.max_depth);

        // fill clock trace with incremental clk values
        self.clk_trace.resize(trace_length, 0);

        for (i, clk) in self.clk_trace.iter_mut().enumerate().skip(self.clk) {
            *clk = i as u128;
        }

        self.clk = self.trace_length() - 1;

        let mut trace: Vec<Vec<u128>> = Vec::new();

        trace.push(self.clk_trace);
        trace.extend(self.registers);

        trace
    }

    fn op_push(&mut self, value: u8) -> Result<(), StackError> {
        self.shift_right(0, 1);
        self.registers[0][self.clk] = value as u128;
        Ok(())
    }

    fn op_read(&mut self) -> Result<(), StackError> {
        self.shift_right(0, 1);
        let value = match self.tape_a.pop() {
            Some(value) => value,
            None => return Err(StackError::empty_inputs(self.clk)),
        };
        self.registers[0][self.clk] = value as u128;
        Ok(())
    }

    fn op_read2(&mut self) -> Result<(), StackError> {
        let ct = match self.tape_b.pop() {
            Some(value) => value.ciphertext(),
            None => return Err(StackError::empty_inputs(self.clk)),
        };

        self.shift_right(0, ct.len());

        for (i, value) in ct.iter().enumerate() {
            self.registers[i][self.clk] = *value;
        }
        Ok(())
    }

    fn op_add(&mut self, op_code: OpCode) -> Result<(), StackError> {
        if self.depth < 2 {
            return Err(StackError::stack_underflow(op_code, self.clk));
        }

        let x = self.registers[0][self.clk - 1];
        let y = self.registers[1][self.clk - 1];
        self.registers[0][self.clk] = x.wrapping_add(y);
        self.shift_left(op_code, 2, 1)
    }

    fn op_sadd(&mut self, op_code: OpCode) -> Result<(), StackError> {
        if self.depth < self.server_key.lwe_size() + 1 {
            return Err(StackError::stack_underflow(op_code, self.clk));
        }

        let ct: Vec<u128> = (1..=self.server_key.lwe_size())
            .map(|i: usize| self.registers[i][self.clk - 1])
            .collect();
        let scalar = self.registers[0][self.clk - 1] as u8;

        let result_ct = self.server_key.scalar_add(&scalar, &FheUInt8::new(&ct));

        for (i, value) in result_ct.ciphertext().iter().enumerate() {
            self.registers[i][self.clk] = *value;
        }

        self.shift_left(op_code, 6, 1)
    }

    fn op_mul(&mut self, op_code: OpCode) -> Result<(), StackError> {
        if self.depth < 2 {
            return Err(StackError::stack_underflow(op_code, self.clk));
        }
        let x = self.registers[0][self.clk - 1];
        let y = self.registers[1][self.clk - 1];
        self.registers[0][self.clk] = x.wrapping_mul(y);
        self.shift_left(op_code, 2, 1)
    }

    fn op_smul(&mut self, op_code: OpCode) -> Result<(), StackError> {
        if self.depth < self.server_key.lwe_size() + 1 {
            return Err(StackError::stack_underflow(op_code, self.clk));
        }

        let ct: Vec<u128> = (1..=self.server_key.lwe_size())
            .map(|i: usize| self.registers[i][self.clk - 1])
            .collect();
        let scalar = self.registers[0][self.clk - 1] as u8;

        let result_ct = self.server_key.scalar_mul(&scalar, &FheUInt8::new(&ct));

        for (i, value) in result_ct.ciphertext().iter().enumerate() {
            self.registers[i][self.clk] = *value;
        }

        self.shift_left(op_code, 6, 1)
    }

    fn shift_left(
        &mut self,
        op_code: OpCode,
        start: usize,
        pos_count: usize,
    ) -> Result<(), StackError> {
        if self.depth < pos_count {
            return Err(StackError::stack_underflow(op_code, self.clk));
        }

        // shift all values by pos_count to the left
        for i in start..self.depth {
            self.registers[i - pos_count][self.clk] = self.registers[i][self.clk - 1];
        }

        // set all "shifted-in" slots to 0
        for i in (self.depth - pos_count)..self.depth {
            self.registers[i][self.clk] = 0;
        }

        // stack depth has been reduced by pos_count
        self.depth -= pos_count;

        Ok(())
    }

    fn shift_right(&mut self, start: usize, pos_count: usize) {
        // stack depth has been increased by pos_count
        self.depth += pos_count;

        // allocate new registers to the stack and increase the stack mac depth
        if self.depth > self.max_depth {
            self.max_depth += pos_count;
            if self.max_depth > self.registers.len() {
                self.add_registers(self.max_depth - self.registers.len());
            }
        }

        // set all "shifted-in" slots to clk' - 1
        for i in start..(self.depth - pos_count) {
            self.registers[i + pos_count][self.clk] = self.registers[i][self.clk - 1];
        }
    }

    /// Extends the stack by the specified number of registers.
    fn add_registers(&mut self, num_registers: usize) {
        for _ in 0..num_registers {
            self.registers.push(vec![0; self.trace_length()]);
        }
    }

    // Ensure there is enough memory allocated for the trace to accommodate a new row.
    // Trace length is doubled every time it needs to be increased.
    // Constrain: trace_length % 2 = 0.
    pub fn ensure_trace_capacity(&mut self) {
        if self.clk >= self.trace_length() {
            let new_length = self.trace_length() * 2;
            for register in self.registers.iter_mut() {
                register.resize(new_length, 0);
            }
            self.clk_trace.resize(new_length, 0);
        }
    }

    // Increment clock by 1
    fn advance_clock(&mut self) {
        self.clk += 1;
        self.clk_trace[self.clk] = self.clk as u128;
    }
}
