use super::errors::ProgramError;
use super::Operation;

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

    Ok(Operation::push(value))
}

pub fn parse_read(op: &[&str], step: usize) -> Result<Operation, ProgramError> {
    if op.len() > 1 {
        return Err(ProgramError::extra_param(op, step));
    }
    Ok(Operation::read())
}

pub fn parse_read2(op: &[&str], step: usize) -> Result<Operation, ProgramError> {
    if op.len() > 1 {
        return Err(ProgramError::extra_param(op, step));
    }
    Ok(Operation::read2())
}

pub fn parse_add(op: &[&str], step: usize) -> Result<Operation, ProgramError> {
    if op.len() > 1 {
        return Err(ProgramError::extra_param(op, step));
    }
    Ok(Operation::add())
}

pub fn parse_mul(op: &[&str], step: usize) -> Result<Operation, ProgramError> {
    if op.len() > 1 {
        return Err(ProgramError::extra_param(op, step));
    }
    Ok(Operation::mul())
}

pub fn parse_sadd(op: &[&str], step: usize) -> Result<Operation, ProgramError> {
    if op.len() > 1 {
        return Err(ProgramError::extra_param(op, step));
    }
    Ok(Operation::sadd())
}

pub fn parse_smul(op: &[&str], step: usize) -> Result<Operation, ProgramError> {
    if op.len() > 1 {
        return Err(ProgramError::extra_param(op, step));
    }
    Ok(Operation::smul())
}

pub fn parse_add2(op: &[&str], step: usize) -> Result<Operation, ProgramError> {
    if op.len() > 1 {
        return Err(ProgramError::extra_param(op, step));
    }
    Ok(Operation::add2())
}
