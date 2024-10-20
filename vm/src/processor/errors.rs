use std::error::Error;

use super::Operation;

#[derive(Debug)]
pub struct StackError {
    message: String,
    step: usize,
}

impl Error for StackError {}

impl StackError {
    pub fn stack_underflow(op: &Operation, step: usize) -> StackError {
        StackError {
            message: format!("{op} operation stack underflow"),
            step,
        }
    }

    pub fn stack_overflow(op: &Operation, step: usize) -> StackError {
        StackError {
            message: format!("{op} operation stack overflow"),
            step,
        }
    }

    pub fn empty_inputs(op: &Operation, step: usize) -> StackError {
        StackError {
            message: format!("no more inputs to {op}"),
            step,
        }
    }
}

impl std::fmt::Display for StackError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "stack error at {}: {}", self.step, self.message)
    }
}

#[derive(Debug)]
pub struct ChipletsError {
    message: String,
    step: usize,
}

impl Error for ChipletsError {}

impl ChipletsError {
    pub fn invalid_operation(op: &Operation, step: usize) -> ChipletsError {
        ChipletsError {
            message: format!("expected noop but was {op}"),
            step,
        }
    }

    pub fn invalid_trace_length(expected: usize, current: usize, step: usize) -> ChipletsError {
        ChipletsError {
            message: format!("trace length should be a multiple of {expected}, but was {current}"),
            step,
        }
    }
}

impl std::fmt::Display for ChipletsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "chiplets error at {}: {}", self.step, self.message)
    }
}

#[derive(Debug)]
pub enum ProcessorError {
    Stack(StackError),
    Chiplets(ChipletsError),
}

impl std::fmt::Display for ProcessorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProcessorError::Stack(e) => write!(f, "{}", e),
            ProcessorError::Chiplets(e) => write!(f, "{}", e),
        }
    }
}
