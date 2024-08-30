pub mod opcodes;
use opcodes::OpCode;

mod errors;
use errors::AssemblyError;

mod parsers;

#[cfg(test)]
mod tests;

pub struct Program {
    code: Vec<OpCode>,
}

impl Program {
    pub fn load(source: &str) -> Result<Program, AssemblyError> {
        let mut code: Vec<OpCode> = Vec::new();

        let tokens: Vec<&str> = source.split_whitespace().collect();

        if tokens.len() == 0 {
            return Err(AssemblyError::empty_program());
        }

        for (i, token) in tokens.iter().enumerate() {
            let instruction = parse_op(i + 1, token)?;

            code.push(instruction);
        }

        Ok(Program { code })
    }
}

fn parse_op(step: usize, line: &str) -> Result<OpCode, AssemblyError> {
    let op: Vec<&str> = line.split(".").collect();

    #[rustfmt::skip]
    return match op[0] {
        "push" => parsers::parse_push(&op, step),
        "read" => parsers::parse_read(&op, step),
        "add" =>  parsers::parse_add(&op, step),
        "mul" =>  parsers::parse_mul(&op, step),
        _ =>      Err(AssemblyError::invalid_op(&op, step)),
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
