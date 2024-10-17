use super::{Operation, ONE, ZERO};
use winterfell::math::fields::f128::BaseElement;

pub struct Decoder {
    clk: usize,
    op_bits_trace: [Vec<BaseElement>; 5],
    trace_length: usize,
}

impl Decoder {
    pub fn new(init_trace_length: usize) -> Decoder {
        let op_bits_trace = [
            vec![ZERO; init_trace_length],
            vec![ZERO; init_trace_length],
            vec![ZERO; init_trace_length],
            vec![ZERO; init_trace_length],
            vec![ZERO; init_trace_length],
        ];

        Decoder {
            clk: 0,
            op_bits_trace,
            trace_length: init_trace_length,
        }
    }

    // pub fn trace_length(&self) -> usize {
    //     self.trace_length
    // }

    #[cfg(test)]
    pub fn decoder_bits_state(&self, clk: usize) -> Vec<BaseElement> {
        let mut state = Vec::with_capacity(self.op_bits_trace.len());
        for i in 0..self.op_bits_trace.len() {
            state.push(self.op_bits_trace[i][clk]);
        }
        state
    }

    pub fn into_trace(mut self, trace_length: usize) -> Vec<Vec<BaseElement>> {
        for col in self.op_bits_trace.iter_mut() {
            col.resize(self.clk + 1, ZERO);
            col.resize(trace_length, ZERO);
        }

        let mut registers: Vec<Vec<BaseElement>> = Vec::new();

        let [b0, b1, b2, b3, b4] = self.op_bits_trace;
        registers.push(b0);
        registers.push(b1);
        registers.push(b2);
        registers.push(b3);
        registers.push(b4);

        registers
    }

    pub fn decode_op(&mut self, op: &Operation) {
        self.advance_clock();
        self.ensure_trace_capacity();
        self.decode_op_bits(op);
    }

    fn advance_clock(&mut self) {
        self.clk += 1;
    }

    fn ensure_trace_capacity(&mut self) {
        if self.clk >= self.trace_length {
            self.trace_length *= 2;
            for col in self.op_bits_trace.iter_mut() {
                col.resize(self.trace_length, ZERO);
            }
        }
    }

    fn decode_op_bits(&mut self, op: &Operation) {
        for i in 0..5 {
            self.op_bits_trace[i][self.clk - 1] = match op.code() >> i & 1 {
                0 => ZERO,
                1 => ONE,
                _ => unreachable!(),
            };
        }
    }
}
