#[derive(Copy, Clone, PartialEq)]
pub enum OpCode {
    Push(u8),
    Read,
    Read2,
    Add,
    SAdd,
    Mul,
    SMul,
}

impl std::fmt::Display for OpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        #[rustfmt::skip]
        return match self {
            OpCode::Push(value) => write!(f, "push({value})"),
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
            OpCode::Push(value) => write!(f, "push({value})"),
            OpCode::Read               => write!(f, "read"),
            OpCode::Read2              => write!(f, "read2"),

            OpCode::Add                => write!(f, "add"),
            OpCode::SAdd               => write!(f, "smul"),
            OpCode::Mul                => write!(f, "mul"),
            OpCode::SMul               => write!(f, "smul"),
        };
    }
}
