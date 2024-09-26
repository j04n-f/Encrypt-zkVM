pub struct System {
    clk: usize,
    clk_trace: Vec<u128>,
}

impl System {
    pub fn new(init_trace_length: usize) -> System {
        let clk_trace = vec![0; init_trace_length];

        System { clk: 0, clk_trace }
    }

    pub fn trace_length(&self) -> usize {
        self.clk_trace.len()
    }

    #[cfg(test)]
    pub fn system_state(&self, clk: usize) -> u128 {
        self.clk_trace[clk]
    }

    pub fn into_trace(mut self, trace_length: usize) -> Vec<Vec<u128>> {
        // fill clock trace with incremental clk values
        self.clk_trace.resize(trace_length, 0);

        for (i, clk) in self.clk_trace.iter_mut().enumerate().skip(self.clk) {
            *clk = i as u128;
        }

        vec![self.clk_trace]
    }

    pub fn advance_step(&mut self) {
        self.advance_clock();
        self.ensure_trace_capacity();

        self.clk_trace[self.clk] = self.clk_trace[self.clk - 1] + 1;
    }

    fn advance_clock(&mut self) {
        self.clk += 1;
    }

    pub fn ensure_trace_capacity(&mut self) {
        if self.clk >= self.trace_length() {
            let new_length = self.trace_length() * 2;
            self.clk_trace.resize(new_length, 0);
        }
    }
}
