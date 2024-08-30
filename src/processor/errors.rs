use super::OpCode;

pub struct StackError {
  message: String,
  step: usize,
  op: String,
}

impl StackError {
  pub fn stack_underflow(op: OpCode, step: usize) -> StackError {
      return StackError {
          message: format!("stack underflow"),
          step: step,
          op: op.to_string(),
      };
  }

  pub fn empty_inputs(step: usize) -> StackError {
    return StackError {
        message: format!("no more inputs to read"),
        step: step,
        op: OpCode::Read.to_string(),
    };
}

  pub fn message(&self) -> &String {
      return &self.message;
  }

  pub fn operation(&self) -> &String {
      return &self.op;
  }

  pub fn step(&self) -> usize {
      return self.step;
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
