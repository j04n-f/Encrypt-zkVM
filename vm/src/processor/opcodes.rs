#[derive(Copy, Clone)]
pub struct Operation {
    op_code: OpCode,
    op_value: OpValue,
}

impl Operation {
    pub fn new(op_code: OpCode, op_value: OpValue) -> Operation {
        Operation { op_code, op_value }
    }

    pub fn code(&self) -> u8 {
        self.op_code as u8
    }

    pub fn value(&self) -> u8 {
        self.op_value.value()
    }

    pub fn op_code(&self) -> OpCode {
        self.op_code
    }

    pub fn op_value(&self) -> OpValue {
        self.op_value
    }
}

#[rustfmt::skip]
#[derive(Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum OpCode {
    Push  = 0b110,    // shift-right: 1
    Read  = 0b111,    // shift-right: 1

    Read2 = 0b11110,    // shift-right: 5

    Add   = 0b101,    // shift-left: 1
    Mul   = 0b100,    // shift-left: 1
    SAdd  = 0b10101,    // shift-left: 1
    SMul  = 0b10110,    // shift-left: 1
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
