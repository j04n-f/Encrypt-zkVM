use std::{fs, path::Path};

use crate::processor::{OpCode, Operation};

mod errors;
use crypto::{
    rescue::{CYCLE_LENGTH, NUM_ROUNDS},
    Hash, Rescue128,
};
use errors::ProgramError;

mod parsers;

pub mod inputs;
pub use inputs::ProgramInputs;

#[cfg(test)]
mod tests;

const PUSH_OP_ALIGNMENT: usize = 8;

#[derive(Debug)]
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

        let comment_symbol = "#";
        let mut tokens = Vec::new();

        for program_line in source.lines() {
            let line = program_line.trim();

            if !line.is_empty() && !line.starts_with(comment_symbol) {
                let mut code_line = match line.find(comment_symbol) {
                    Some(pos) => &line[..pos],
                    None => line,
                };

                code_line = code_line.trim();

                if !code_line.is_empty() {
                    tokens.push(code_line);
                }
            }
        }

        if tokens.is_empty() {
            return Err(ProgramError::empty_program());
        }

        for (i, token) in tokens.iter().enumerate() {
            let op = parse_op(i + 1, token)?;

            if let OpCode::Push = op.op_code() {
                let alignment = code.len() % PUSH_OP_ALIGNMENT;
                let pad_length = (PUSH_OP_ALIGNMENT - alignment) % PUSH_OP_ALIGNMENT;
                code.resize(code.len() + pad_length, Operation::noop());
            }

            // add NOOP codes when CYCLE_LENGTH >= NUM_ROUNDS
            // to reset the capacity elements to 0
            if code.len() % CYCLE_LENGTH >= NUM_ROUNDS {
                let padded_length = compute_padding(code.len());
                code.resize(padded_length, Operation::noop());
            }

            code.push(op);
        }

        // pad the program length with NOOP codes to match the RescuePrime cycle length
        let padded_length = compute_padding(code.len());
        code.resize(padded_length, Operation::noop());

        for op in code.iter() {
            sponge.update(op.code(), op.value());
        }

        Ok(Program {
            code,
            hash: sponge.hash(),
        })
    }

    pub fn code(&self) -> &[Operation] {
        &self.code
    }

    pub fn hash(&self) -> Hash {
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
        "mul"   => parsers::parse_mul(&op, step),
        "sadd"  => parsers::parse_sadd(&op, step),
        "smul"  => parsers::parse_smul(&op, step),
        "add2"  => parsers::parse_add2(&op, step),
        _       => Err(ProgramError::invalid_op(&op, step)),
    };
}

fn compute_padding(length: usize) -> usize {
    length + (CYCLE_LENGTH - (length % CYCLE_LENGTH))
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
