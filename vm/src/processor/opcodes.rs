pub type Operation = (OpCode, OpValue);

#[rustfmt::skip]
#[derive(Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum OpCode {
    Push  = 0b0110_0000,
    Read  = 0b0111_0000,
    Read2 = 0b0111_0001,
    Add   = 0b0111_0010,
    SAdd  = 0b0111_0011,
    Mul   = 0b0111_0100,
    SMul  = 0b0111_0101,
}

impl std::fmt::Display for OpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        #[rustfmt::skip]
        return match self {
            OpCode::Push               => write!(f, "push"),
            OpCode::Read               => write!(f, "read"),
            OpCode::Read2              => write!(f, "read2"),

            OpCode::Add                => write!(f, "add"),
            OpCode::SAdd               => write!(f, "smul"),
            OpCode::Mul                => write!(f, "mul"),
            OpCode::SMul               => write!(f, "smul"),
        };
    }
}

impl std::fmt::Debug for OpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        #[rustfmt::skip]
        return match self {
            OpCode::Push               => write!(f, "push"),
            OpCode::Read               => write!(f, "read"),
            OpCode::Read2              => write!(f, "read2"),

            OpCode::Add                => write!(f, "add"),
            OpCode::SAdd               => write!(f, "smul"),
            OpCode::Mul                => write!(f, "mul"),
            OpCode::SMul               => write!(f, "smul"),
        };
    }
}

#[derive(Copy, Clone)]
pub enum OpValue {
    Push(u8),
    None,
}

impl OpValue {
    pub fn value(&self) -> u8 {
        match self {
            OpValue::Push(value) => *value,
            _ => 0,
        }
    }
}

impl std::fmt::Display for OpValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            OpValue::Push(value) => write!(f, "({})", value),
            OpValue::None => Ok(()),
        }
    }
}

impl std::fmt::Debug for OpValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            OpValue::Push(value) => write!(f, "({})", value),
            OpValue::None => Ok(()),
        }
    }
}
