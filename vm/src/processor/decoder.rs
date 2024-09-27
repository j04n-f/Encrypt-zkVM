use super::{Operation, ONE, ZERO};
use crypto::{Rescue128, STATE_WIDTH};
use winterfell::math::fields::f128::BaseElement;

pub struct Decoder {
    clk: usize,
    sponge: Rescue128,
    sponge_trace: [Vec<BaseElement>; STATE_WIDTH],
    op_bits_trace: [Vec<BaseElement>; 3],
    trace_length: usize,
}

impl Decoder {
    pub fn new(init_trace_length: usize) -> Decoder {
        let op_bits_trace = [
            vec![ZERO; init_trace_length],
            vec![ZERO; init_trace_length],
            vec![ZERO; init_trace_length],
        ];

        let sponge_trace = [
            vec![ZERO; init_trace_length],
            vec![ZERO; init_trace_length],
            vec![ZERO; init_trace_length],
            vec![ZERO; init_trace_length],
            vec![ZERO; init_trace_length],
            vec![ZERO; init_trace_length],
        ];

        let sponge = Rescue128::new();

        Decoder {
            clk: 0,
            sponge,
            sponge_trace,
            op_bits_trace,
            trace_length: init_trace_length,
        }
    }

    pub fn trace_length(&self) -> usize {
        self.trace_length
    }

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

        for col in self.sponge_trace.iter_mut() {
            col.resize(self.clk + 1, ZERO);
            col.resize(trace_length, col[self.clk]);
        }

        let mut registers: Vec<Vec<BaseElement>> = Vec::new();

        let [r0, r1, r2] = self.op_bits_trace;
        registers.push(r0);
        registers.push(r1);
        registers.push(r2);

        let [h0, h1, h2, h3, h4, h5] = self.sponge_trace;
        registers.push(h0);
        registers.push(h1);
        registers.push(h2);
        registers.push(h3);
        registers.push(h4);
        registers.push(h5);

        registers
    }

    pub fn decode_op(&mut self, op: &Operation) {
        self.advance_clock();
        self.ensure_trace_capacity();
        self.set_bits(op.code());
        self.set_state(op.code(), op.value())
    }

    fn advance_clock(&mut self) {
        self.clk += 1;
    }

    pub fn ensure_trace_capacity(&mut self) {
        if self.clk >= self.trace_length {
            self.trace_length *= 2;
            for col in self.op_bits_trace.iter_mut() {
                col.resize(self.trace_length, ZERO);
            }
            for col in self.sponge_trace.iter_mut() {
                col.resize(self.trace_length, ZERO);
            }
        }
    }

    fn set_bits(&mut self, code: u8) {
        let clk = self.clk - 1;

        for i in 0..3 {
            self.op_bits_trace[i][clk] = match code >> i & 1 {
                0 => ZERO,
                1 => ONE,
                _ => unreachable!(),
            };
        }
    }

    fn set_state(&mut self, code: u8, value: u8) {
        let clk = self.clk - 1;

        self.sponge
            .update(&[BaseElement::from(code), BaseElement::from(value), ZERO, ZERO]);

        for (i, state) in self.sponge.state().iter().enumerate() {
            self.sponge_trace[i][clk] = *state;
        }
    }
}
