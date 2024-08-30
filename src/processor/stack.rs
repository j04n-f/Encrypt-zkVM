use super::OpCode;
use super::ProgramInputs;
use super::StackError;

const MIN_STACK_DEPTH: usize = 5;
const MIN_TRACE_LENGTH: usize = 16;

pub struct Stack {
    registers: Vec<Vec<u128>>,
    tape_a: Vec<u128>,
    max_depth: usize,
    depth: usize,
    step: usize,
}

impl Stack {
    pub fn new(inputs: &ProgramInputs) -> Stack {
        let mut registers: Vec<Vec<u128>> = Vec::with_capacity(MIN_STACK_DEPTH);

        for _ in 0..MIN_STACK_DEPTH {
            registers.push(vec![0; MIN_TRACE_LENGTH]);
        }

        let mut tape_a = inputs.get_values().to_vec();
        tape_a.reverse();

        Stack {
            registers,
            tape_a,
            max_depth: 0,
            depth: 0,
            step: 0,
        }
    }

    pub fn execute(&mut self, op_code: OpCode) -> Result<(), StackError> {
        self.advance_step();

        #[rustfmt::skip]
        return match op_code {
            OpCode::Push(value) => self.op_push(value),
            OpCode::Read              => self.op_read(),

            OpCode::Add               => self.op_add(op_code),
            OpCode::Mul               => self.op_mul(op_code),
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

    pub fn get_stack_top(&self) -> u128 {
        self.registers[0][self.step]
    }

    fn op_push(&mut self, value: u128) -> Result<(), StackError> {
        self.shift_right(0, 1);
        self.registers[0][self.step] = value;
        Ok(())
    }

    fn op_read(&mut self) -> Result<(), StackError> {
        self.shift_right(0, 1);
        let value = match self.tape_a.pop() {
            Some(value) => value,
            None => return Err(StackError::empty_inputs(self.step)),
        };
        self.registers[0][self.step] = value;
        Ok(())
    }

    fn op_add(&mut self, op_code: OpCode) -> Result<(), StackError> {
        if self.depth < 2 {
            return Err(StackError::stack_underflow(op_code, self.step));
        }

        let x = self.registers[0][self.step - 1];
        let y = self.registers[1][self.step - 1];
        self.registers[0][self.step] = x + y;
        self.shift_left(op_code, 2, 1)
    }

    fn op_mul(&mut self, op_code: OpCode) -> Result<(), StackError> {
        if self.depth < 2 {
            return Err(StackError::stack_underflow(op_code, self.step));
        }
        assert!(self.depth >= 2, "stack underflow at step {}", self.step);
        let x = self.registers[0][self.step - 1];
        let y = self.registers[1][self.step - 1];
        self.registers[0][self.step] = x * y;
        self.shift_left(op_code, 2, 1)
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
