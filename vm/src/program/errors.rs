pub struct ProgramError {
    message: String,
    step: usize,
}

impl ProgramError {
    pub fn read_error(message: &str) -> ProgramError {
        ProgramError {
            message: message.to_string(),
            step: 0,
        }
    }

    pub fn empty_program() -> ProgramError {
        ProgramError {
            message: String::from("a program must contain at least one instruction"),
            step: 0,
        }
    }

    pub fn invalid_op(op: &[&str], step: usize) -> ProgramError {
        ProgramError {
            message: format!("instruction {} is invalid", op.join(".")),
            step,
        }
    }

    pub fn missing_param(op: &[&str], step: usize) -> ProgramError {
        ProgramError {
            message: format!("malformed instruction {}, parameter is missing", op[0]),
            step,
        }
    }

    pub fn extra_param(op: &[&str], step: usize) -> ProgramError {
        ProgramError {
            message: format!(
                "malformed instruction {}, too many parameters provided",
                op[0]
            ),
            step,
        }
    }

    pub fn invalid_param(op: &[&str], step: usize) -> ProgramError {
        ProgramError {
            message: format!(
                "malformed instruction {}, parameter '{}' is invalid",
                op[0], op[1]
            ),
            step,
        }
    }
}

impl std::fmt::Debug for ProgramError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "program error at {}: {}", self.step, self.message)
    }
}

impl std::fmt::Display for ProgramError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "program error at {}: {}", self.step, self.message)
    }
}
