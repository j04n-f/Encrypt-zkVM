pub struct AssemblyError {
    message: String,
    step: usize,
    op: String,
}

impl AssemblyError {
    pub fn empty_program() -> AssemblyError {
        return AssemblyError {
            message: String::from("a program must contain at least one instruction"),
            step: 0,
            op: String::from("begin"),
        };
    }

    pub fn invalid_op(op: &[&str], step: usize) -> AssemblyError {
        return AssemblyError {
            message: format!("instruction {} is invalid", op.join(".")),
            step: step,
            op: op.join("."),
        };
    }

    pub fn missing_param(op: &[&str], step: usize) -> AssemblyError {
        return AssemblyError {
            message: format!("malformed instruction {}, parameter is missing", op[0]),
            step: step,
            op: op.join("."),
        };
    }

    pub fn extra_param(op: &[&str], step: usize) -> AssemblyError {
        return AssemblyError {
            message: format!(
                "malformed instruction {}, too many parameters provided",
                op[0]
            ),
            step: step,
            op: op.join("."),
        };
    }

    pub fn invalid_param(op: &[&str], step: usize) -> AssemblyError {
        return AssemblyError {
            message: format!(
                "malformed instruction {}, parameter '{}' is invalid",
                op[0], op[1]
            ),
            step: step,
            op: op.join("."),
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

impl std::fmt::Debug for AssemblyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "assembly error at {}: {}", self.step, self.message)
    }
}

impl std::fmt::Display for AssemblyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "assembly error at {}: {}", self.step, self.message)
    }
}
