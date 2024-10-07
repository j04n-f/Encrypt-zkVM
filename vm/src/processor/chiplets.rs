use super::{HashOperation, Operation, ONE, ZERO};
use crypto::{
    rescue,
    rescue::{CYCLE_LENGTH, NUM_ROUNDS, STATE_WIDTH},
};
use winterfell::math::{fields::f128::BaseElement, FieldElement};

pub struct Chiplets {
    clk: usize,
    sponge: [BaseElement; STATE_WIDTH],
    op_bits_trace: [Vec<BaseElement>; 1],
    sponge_trace: [Vec<BaseElement>; STATE_WIDTH],
    trace_length: usize,
}

impl Chiplets {
    pub fn new(init_trace_length: usize) -> Chiplets {
        let sponge_trace = [
            vec![ZERO; init_trace_length],
            vec![ZERO; init_trace_length],
            vec![ZERO; init_trace_length],
            vec![ZERO; init_trace_length],
        ];

        let op_bits_trace = [vec![ZERO; init_trace_length]];

        let sponge = [BaseElement::ZERO; STATE_WIDTH];

        Chiplets {
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

    pub fn into_trace(mut self, trace_length: usize) -> Vec<Vec<BaseElement>> {
        for col in self.sponge_trace.iter_mut() {
            col.resize(self.clk + 1, ZERO);
            col.resize(trace_length, col[self.clk]);
        }

        for col in self.op_bits_trace.iter_mut() {
            col.resize(self.clk + 1, ZERO);
            col.resize(trace_length, ZERO);
        }

        let mut registers: Vec<Vec<BaseElement>> = Vec::new();

        let [b0] = self.op_bits_trace;
        registers.push(b0);

        let [h0, h1, h2, h3] = self.sponge_trace;
        registers.push(h0);
        registers.push(h1);
        registers.push(h2);
        registers.push(h3);

        registers
    }

    pub fn hash_op(&mut self, op: &Operation) {
        self.advance_clock();
        self.ensure_trace_capacity();
        self.apply_hacc_round(op);
    }

    fn advance_clock(&mut self) {
        self.clk += 1;
    }

    fn ensure_trace_capacity(&mut self) {
        if self.clk >= self.trace_length {
            self.trace_length *= 2;
            for col in self.sponge_trace.iter_mut() {
                col.resize(self.trace_length, ZERO);
            }
            for col in self.op_bits_trace.iter_mut() {
                col.resize(self.trace_length, ZERO);
            }
        }
    }

    fn apply_hacc_round(&mut self, op: &Operation) {
        if (self.clk - 1) % CYCLE_LENGTH < NUM_ROUNDS {
            rescue::apply_round(&mut self.sponge, op.code(), op.value(), self.clk - 1);
        } else {
            self.sponge[2] = ZERO;
            self.sponge[3] = ZERO;
        }

        let hash_op = HashOperation::round();

        self.op_bits_trace[0][self.clk - 1] = match hash_op.code() >> 0 & 1 {
            0 => ZERO,
            1 => ONE,
            _ => unreachable!(),
        };

        for (col, state) in self.sponge.iter().enumerate() {
            self.sponge_trace[col][self.clk] = *state;
        }
    }
}
