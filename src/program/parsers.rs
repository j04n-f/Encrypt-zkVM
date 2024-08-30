use super::errors::AssemblyError;
use super::OpCode;

pub fn parse_push(op: &[&str], step: usize) -> Result<OpCode, AssemblyError> {
    if op.len() == 1 {
        return Err(AssemblyError::missing_param(op, step));
    } else if op.len() > 2 {
        return Err(AssemblyError::extra_param(op, step));
    }

    let value = match u128::from_str_radix(&op[1], 10) {
        Ok(i) => i,
        Err(_) => return Err(AssemblyError::invalid_param(op, step)),
    };

    Ok(OpCode::Push(value))
}

pub fn parse_read(op: &[&str], step: usize) -> Result<OpCode, AssemblyError> {
    if op.len() > 1 {
        return Err(AssemblyError::extra_param(op, step));
    }

    Ok(OpCode::Read)
}

pub fn parse_add(op: &[&str], step: usize) -> Result<OpCode, AssemblyError> {
    if op.len() > 1 {
        return Err(AssemblyError::extra_param(op, step));
    }
    Ok(OpCode::Add)
}

pub fn parse_mul(op: &[&str], step: usize) -> Result<OpCode, AssemblyError> {
    if op.len() > 1 {
        return Err(AssemblyError::extra_param(op, step));
    }
    Ok(OpCode::Mul)
}
