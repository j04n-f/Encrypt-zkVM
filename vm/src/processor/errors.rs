use std::error::Error;

pub struct StackError {
    message: String,
    step: usize,
}

impl Error for StackError {}

impl StackError {
    pub fn stack_underflow(op: &str, step: usize) -> StackError {
        StackError {
            message: format!("{op} operation stack underflow"),
            step,
        }
    }

    pub fn stack_overflow(op: &str, step: usize) -> StackError {
        StackError {
            message: format!("{op} operation stack overflow"),
            step,
        }
    }

    pub fn empty_inputs(step: usize) -> StackError {
        StackError {
            message: "no more inputs to read".to_string(),
            step,
        }
    }
}

impl std::fmt::Debug for StackError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "stack error at {}: {}", self.step, self.message)
    }
}

impl std::fmt::Display for StackError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "stack error at {}: {}", self.step, self.message)
    }
}
