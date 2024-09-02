use super::OpCode;
use super::ProgramInputs;
use super::StackError;

use fhe::{FheUInt8, ServerKey};

const MIN_STACK_DEPTH: usize = 5;
const MIN_TRACE_LENGTH: usize = 16;

pub struct Stack {
    registers: Vec<Vec<u128>>,
    tape_a: Vec<u8>,
    tape_b: Vec<FheUInt8>,
    max_depth: usize,
    depth: usize,
    step: usize,
    server_key: ServerKey,
}

impl Stack {
    pub fn new(inputs: &ProgramInputs) -> Stack {
        let mut registers: Vec<Vec<u128>> = Vec::with_capacity(MIN_STACK_DEPTH);

        for _ in 0..MIN_STACK_DEPTH {
            registers.push(vec![0; MIN_TRACE_LENGTH]);
        }

        let mut tape_a = inputs.get_public();
        tape_a.reverse();
        let mut tape_b = inputs.get_secret();
        tape_b.reverse();

        Stack {
            registers,
            tape_a,
            tape_b,
            max_depth: 0,
            depth: 0,
            step: 0,
            server_key: inputs.get_server_key(),
        }
    }

    pub fn execute(&mut self, op_code: OpCode) -> Result<(), StackError> {
        self.advance_step();

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
    pub fn get_stack_state(&self, step: usize) -> Vec<u128> {
        let mut state = Vec::with_capacity(self.registers.len());
        for i in 0..self.registers.len() {
            state.push(self.registers[i][step]);
        }
        state
    }

    pub fn get_current_state(&self) -> Vec<u128> {
        let mut state = Vec::with_capacity(self.registers.len());
        for i in 0..self.registers.len() {
            state.push(self.registers[i][self.step]);
        }
        state
    }

    fn op_push(&mut self, value: u8) -> Result<(), StackError> {
        self.shift_right(0, 1);
        self.registers[0][self.step] = value as u128;
        Ok(())
    }

    fn op_read(&mut self) -> Result<(), StackError> {
        self.shift_right(0, 1);
        let value = match self.tape_a.pop() {
            Some(value) => value,
            None => return Err(StackError::empty_inputs(self.step)),
        };
        self.registers[0][self.step] = value as u128;
        Ok(())
    }

    fn op_read2(&mut self) -> Result<(), StackError> {
        let ct = match self.tape_b.pop() {
            Some(value) => value.ciphertext(),
            None => return Err(StackError::empty_inputs(self.step)),
        };

        self.shift_right(0, ct.len());

        for (i, value) in ct.iter().enumerate() {
            self.registers[i][self.step] = *value;
        }
        Ok(())
    }

    fn op_add(&mut self, op_code: OpCode) -> Result<(), StackError> {
        if self.depth < 2 {
            return Err(StackError::stack_underflow(op_code, self.step));
        }

        let x = self.registers[0][self.step - 1];
        let y = self.registers[1][self.step - 1];
        self.registers[0][self.step] = x.wrapping_add(y);
        self.shift_left(op_code, 2, 1)
    }

    fn op_sadd(&mut self, op_code: OpCode) -> Result<(), StackError> {
        if self.depth < self.server_key.lwe_size() + 1 {
            return Err(StackError::stack_underflow(op_code, self.step));
        }

        let ct: Vec<u128> = (1..=self.server_key.lwe_size())
            .map(|i: usize| self.registers[i][self.step - 1])
            .collect();
        let scalar = self.registers[0][self.step - 1] as u8;

        let result_ct = self.server_key.scalar_add(&scalar, &FheUInt8::new(&ct));

        for (i, value) in result_ct.ciphertext().iter().enumerate() {
            self.registers[i][self.step] = *value;
        }

        self.shift_left(op_code, 6, 1)
    }

    fn op_mul(&mut self, op_code: OpCode) -> Result<(), StackError> {
        if self.depth < 2 {
            return Err(StackError::stack_underflow(op_code, self.step));
        }
        let x = self.registers[0][self.step - 1];
        let y = self.registers[1][self.step - 1];
        self.registers[0][self.step] = x.wrapping_mul(y);
        self.shift_left(op_code, 2, 1)
    }

    fn op_smul(&mut self, op_code: OpCode) -> Result<(), StackError> {
        if self.depth < self.server_key.lwe_size() + 1 {
            return Err(StackError::stack_underflow(op_code, self.step));
        }

        let ct: Vec<u128> = (1..=self.server_key.lwe_size())
            .map(|i: usize| self.registers[i][self.step - 1])
            .collect();
        let scalar = self.registers[0][self.step - 1] as u8;

        let result_ct = self.server_key.scalar_mul(&scalar, &FheUInt8::new(&ct));

        for (i, value) in result_ct.ciphertext().iter().enumerate() {
            self.registers[i][self.step] = *value;
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
            return Err(StackError::stack_underflow(op_code, self.step));
        }

        // shift all values by pos_count to the left
        for i in start..self.depth {
            self.registers[i - pos_count][self.step] = self.registers[i][self.step - 1];
        }

        // set all "shifted-in" slots to 0
        for i in (self.depth - pos_count)..self.depth {
            self.registers[i][self.step] = 0;
        }

        // stack depth has been reduced by pos_count
        self.depth -= pos_count;

        Ok(())
    }

    fn shift_right(&mut self, start: usize, pos_count: usize) {
        self.depth += pos_count;

        if self.depth > self.max_depth {
            self.max_depth += pos_count;
            if self.max_depth > self.registers.len() {
                self.add_registers(self.max_depth - self.registers.len());
            }
        }

        for i in start..(self.depth - pos_count) {
            self.registers[i + pos_count][self.step] = self.registers[i][self.step - 1];
        }
    }

    /// Extends the stack by the specified number of registers.
    fn add_registers(&mut self, num_registers: usize) {
        for _ in 0..num_registers {
            self.registers.push(vec![0; self.trace_length()]);
        }
    }

    fn advance_step(&mut self) {
        // increment step by 1
        self.step += 1;

        // make sure there is enough memory allocated for register traces
        if self.step >= self.trace_length() {
            let new_length = self.trace_length() * 2;
            for register in self.registers.iter_mut() {
                register.resize(new_length, 0);
            }
        }
    }
}
