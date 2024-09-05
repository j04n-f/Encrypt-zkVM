use std::{fs, path::Path};

use crate::processor::OpCode;

mod errors;
use errors::ProgramError;

mod parsers;

pub mod inputs;
pub use inputs::ProgramInputs;

#[cfg(test)]
mod tests;

pub struct Program {
    code: Vec<OpCode>,
}

impl Program {
    pub fn load(path: &Path) -> Result<Program, ProgramError> {
        let source = match fs::read_to_string(path) {
            Ok(source) => source,
            Err(err) => return Err(ProgramError::read_error(&err.to_string().to_lowercase())),
        };
        Program::compile(&source)
    }

    pub fn compile(source: &str) -> Result<Program, ProgramError> {
        let mut code: Vec<OpCode> = Vec::new();

        let tokens: Vec<&str> = source.split_whitespace().collect();

        if tokens.is_empty() {
            return Err(ProgramError::empty_program());
        }

        for (i, token) in tokens.iter().enumerate() {
            let instruction = parse_op(i + 1, token)?;

            code.push(instruction);
        }

        Ok(Program { code })
    }

    pub fn get_code(&self) -> &[OpCode] {
        &self.code
    }
}

fn parse_op(step: usize, line: &str) -> Result<OpCode, ProgramError> {
    let op: Vec<&str> = line.split('.').collect();

    #[rustfmt::skip]
    return match op[0] {
        "push"  => parsers::parse_push(&op, step),
        "read"  => parsers::parse_read(&op, step),
        "read2" => parsers::parse_read2(&op, step),
        "add"   => parsers::parse_add(&op, step),
        "sadd"  => parsers::parse_sadd(&op, step),
        "mul"   => parsers::parse_mul(&op, step),
        "smul"  => parsers::parse_smul(&op, step),
        _       => Err(ProgramError::invalid_op(&op, step)),
    };
}

impl std::fmt::Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.code[0])?;

        for i in 1..self.code.len() {
            write!(f, " {}", self.code[i])?;
        }

        Ok(())
    }
}

impl std::fmt::Debug for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.code[0])?;

        for i in 1..self.code.len() {
            write!(f, " {}", self.code[i])?;
        }

        Ok(())
    }
}
