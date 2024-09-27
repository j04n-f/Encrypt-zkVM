use std::{fs, path::Path};

use crate::processor::{OpCode, OpValue, Operation};

mod errors;
use crypto::{Hash, Rescue128};
use errors::ProgramError;

mod parsers;

pub mod inputs;
pub use inputs::ProgramInputs;

use winterfell::math::{fields::f128::BaseElement, FieldElement};

#[cfg(test)]
mod tests;

pub struct Program {
    code: Vec<Operation>,
    hash: Hash,
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
        let mut code: Vec<Operation> = Vec::new();
        let mut sponge = Rescue128::new();

        let tokens: Vec<&str> = source.split_whitespace().collect();

        if tokens.is_empty() {
            return Err(ProgramError::empty_program());
        }

        for (i, token) in tokens.iter().enumerate() {
            let op = parse_op(i + 1, token)?;

            // TODO: Change Rescue-Prime to absorb 2 elements per round
            sponge.update(&[
                BaseElement::from(op.code()),
                BaseElement::from(op.value()),
                BaseElement::ZERO,
                BaseElement::ZERO,
            ]);

            code.push(op);
        }

        let hash = sponge.finalize();

        Ok(Program { code, hash })
    }

    pub fn get_code(&self) -> &[Operation] {
        &self.code
    }

    pub fn get_hash(&self) -> Hash {
        self.hash
    }
}

fn parse_op(step: usize, line: &str) -> Result<Operation, ProgramError> {
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
        write!(f, "{}{}", self.code[0].op_code(), self.code[0].op_value())?;

        for i in 1..self.code.len() {
            write!(f, " {}{}", self.code[i].op_code(), self.code[i].op_value())?;
        }

        Ok(())
    }
}

impl std::fmt::Debug for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}{}", self.code[0].op_code(), self.code[0].op_value())?;

        for i in 1..self.code.len() {
            write!(f, " {}{}", self.code[i].op_code(), self.code[i].op_value())?;
        }

        Ok(())
    }
}
