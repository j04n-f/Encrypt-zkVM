#[derive(Clone, Debug)]
pub struct ProgramInputs {
    inputs: Vec<u128>,
}

impl ProgramInputs {
    /// Returns `ProgramInputs` initialized with the provided public and secret inputs.
    pub fn new(inputs: &[u128]) -> ProgramInputs {
        return ProgramInputs {
            inputs: inputs.to_vec(),
        };
    }

    pub fn none() -> ProgramInputs {
        ProgramInputs { inputs: vec![] }
    }

    pub fn get_values(&self) -> &[u128] {
        &self.inputs
    }
}
