use super::{OpCode, Operation};

pub struct Decoder {
    clk: usize,
    op_bits_registers: [Vec<u128>; 3],
}

impl Decoder {
    pub fn new(init_trace_length: usize) -> Decoder {
        let op_bits_registers = [
            vec![0; init_trace_length],
            vec![0; init_trace_length],
            vec![0; init_trace_length],
        ];

        Decoder {
            clk: 0,
            op_bits_registers,
        }
    }

    pub fn trace_length(&self) -> usize {
        self.op_bits_registers[0].len()
    }

    #[cfg(test)]
    pub fn decoder_state(&self, clk: usize) -> Vec<u128> {
        let mut state = Vec::with_capacity(self.op_bits_registers.len());
        for i in 0..self.op_bits_registers.len() {
            state.push(self.op_bits_registers[i][clk]);
        }
        state
    }

    pub fn into_trace(mut self) -> Vec<Vec<u128>> {
        let trace_length = self.trace_length();

        for op_bits_register in self.op_bits_registers.iter_mut() {
            op_bits_register.resize(self.clk + 1, 0);
            op_bits_register.resize(trace_length, 0);
        }

        let mut registers: Vec<Vec<u128>> = Vec::new();

        let [r0, r1, r2] = self.op_bits_registers;
        registers.push(r0);
        registers.push(r1);
        registers.push(r2);

        registers
    }

    pub fn decode_op(&mut self, op: &Operation) {
        self.advance_clock();
        self.ensure_trace_capacity();
        self.set_op_bits(op.0);
    }

    fn advance_clock(&mut self) {
        self.clk += 1;
    }

    pub fn ensure_trace_capacity(&mut self) {
        if self.clk >= self.trace_length() {
            let new_length = self.trace_length() * 2;

            for register in self.op_bits_registers.iter_mut() {
                register.resize(new_length, 0);
            }
        }
    }

    fn set_op_bits(&mut self, op_code: OpCode) {
        let clk = self.clk - 1;

        let op_code = op_code as u8;

        for i in 0..3 {
            let bit = (op_code >> i & 1) as u128;
            self.op_bits_registers[i][clk] = bit;
        }
    }
}
