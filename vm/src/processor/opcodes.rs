#[rustfmt::skip]
#[derive(Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum HashCode {
    Round  = 0b1,
}

#[derive(Copy, Clone)]
pub struct HashOperation {
    hash_code: HashCode,
}

impl HashOperation {
    pub fn new(hash_code: HashCode) -> HashOperation {
        HashOperation { hash_code }
    }

    pub fn code(&self) -> u8 {
        self.hash_code as u8
    }

    pub fn round() -> HashOperation {
        HashOperation::new(HashCode::Round)
    }
}

#[rustfmt::skip]
#[derive(Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum OpCode {
    Noop  = 0b00_000,

    Push  = 0b10_000,    // shift-right: 1
    Read  = 0b10_001,    // shift-right: 1

    Read2 = 0b10_010,    // shift-right: 5

    Add   = 0b01_000,    // shift-left: 1
    Mul   = 0b01_001,    // shift-left: 1
    SAdd  = 0b01_010,    // shift-left: 1
    SMul  = 0b01_100,    // shift-left: 1
}

impl std::fmt::Display for OpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        #[rustfmt::skip]
        return match self {
            OpCode::Noop               => write!(f, "noop"),

            OpCode::Push               => write!(f, "push"),
            OpCode::Read               => write!(f, "read"),
            OpCode::Read2              => write!(f, "read2"),

            OpCode::Add                => write!(f, "add"),
            OpCode::SAdd               => write!(f, "sadd"),
            OpCode::Mul                => write!(f, "mul"),
            OpCode::SMul               => write!(f, "smul"),
        };
    }
}

impl std::fmt::Debug for OpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        #[rustfmt::skip]
        return match self {
            OpCode::Noop               => write!(f, "noop"),

            OpCode::Push               => write!(f, "push"),
            OpCode::Read               => write!(f, "read"),
            OpCode::Read2              => write!(f, "read2"),

            OpCode::Add                => write!(f, "add"),
            OpCode::SAdd               => write!(f, "sadd"),
            OpCode::Mul                => write!(f, "mul"),
            OpCode::SMul               => write!(f, "smul"),
        };
    }
}

#[derive(Copy, Clone, PartialEq)]
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
            OpValue::Push(value) => write!(f, "({:?})", value),
            OpValue::None => Ok(()),
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
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

    pub fn noop() -> Operation {
        Operation::new(OpCode::Noop, OpValue::None)
    }

    pub fn push(value: u8) -> Operation {
        Operation::new(OpCode::Push, OpValue::Push(value))
    }

    pub fn read() -> Operation {
        Operation::new(OpCode::Read, OpValue::None)
    }

    pub fn read2() -> Operation {
        Operation::new(OpCode::Read2, OpValue::None)
    }

    pub fn add() -> Operation {
        Operation::new(OpCode::Add, OpValue::None)
    }

    pub fn mul() -> Operation {
        Operation::new(OpCode::Mul, OpValue::None)
    }

    pub fn sadd() -> Operation {
        Operation::new(OpCode::SAdd, OpValue::None)
    }

    pub fn smul() -> Operation {
        Operation::new(OpCode::SMul, OpValue::None)
    }
}

impl std::fmt::Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}{}", self.op_code, self.op_value)
    }
}

impl std::fmt::Debug for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}{:?}", self.op_code, self.op_value)
    }
}
