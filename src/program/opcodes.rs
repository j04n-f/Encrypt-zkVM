#[repr(u8)]
#[derive(Copy, Clone, PartialEq)]
pub enum OpCode {
    Push(u128),
    Read,
    Add,
    Mul,
}

impl std::fmt::Display for OpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        #[rustfmt::skip]
        return match self {
            OpCode::Push(value) => write!(f, "push({value})"),
            OpCode::Read =>               write!(f, "read"),

            OpCode::Add =>                write!(f, "add"),
            OpCode::Mul =>                write!(f, "mul"),
        };
    }
}

impl std::fmt::Debug for OpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        #[rustfmt::skip]
        return match self {
            OpCode::Push(value) => write!(f, "push({value})"),
            OpCode::Read =>               write!(f, "read"),

            OpCode::Add =>                write!(f, "add"),
            OpCode::Mul =>                write!(f, "mul"),
        };
    }
}
