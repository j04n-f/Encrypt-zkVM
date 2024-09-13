use super::errors::ProgramError;
use super::{OpCode, OpValue, Operation};

pub fn parse_push(op: &[&str], step: usize) -> Result<Operation, ProgramError> {
    if op.len() == 1 {
        return Err(ProgramError::missing_param(op, step));
    } else if op.len() > 2 {
        return Err(ProgramError::extra_param(op, step));
    }

    let value = match op[1].parse::<u8>() {
        Ok(i) => i,
        Err(_) => return Err(ProgramError::invalid_param(op, step)),
    };

    Ok((OpCode::Push, OpValue::Push(value)))
}

pub fn parse_read(op: &[&str], step: usize) -> Result<Operation, ProgramError> {
    if op.len() > 1 {
        return Err(ProgramError::extra_param(op, step));
    }
    Ok((OpCode::Read, OpValue::None))
}

pub fn parse_read2(op: &[&str], step: usize) -> Result<Operation, ProgramError> {
    if op.len() > 1 {
        return Err(ProgramError::extra_param(op, step));
    }
    Ok((OpCode::Read2, OpValue::None))
}

pub fn parse_add(op: &[&str], step: usize) -> Result<Operation, ProgramError> {
    if op.len() > 1 {
        return Err(ProgramError::extra_param(op, step));
    }
    Ok((OpCode::Add, OpValue::None))
}

pub fn parse_sadd(op: &[&str], step: usize) -> Result<Operation, ProgramError> {
    if op.len() > 1 {
        return Err(ProgramError::extra_param(op, step));
    }
    Ok((OpCode::SAdd, OpValue::None))
}

pub fn parse_mul(op: &[&str], step: usize) -> Result<Operation, ProgramError> {
    if op.len() > 1 {
        return Err(ProgramError::extra_param(op, step));
    }
    Ok((OpCode::Mul, OpValue::None))
}

pub fn parse_smul(op: &[&str], step: usize) -> Result<Operation, ProgramError> {
    if op.len() > 1 {
        return Err(ProgramError::extra_param(op, step));
    }
    Ok((OpCode::SMul, OpValue::None))
}
