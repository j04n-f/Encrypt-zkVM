use winterfell::{math::FieldElement, EvaluationFrame};

pub trait EvaluationFrameExtBits<E: FieldElement> {
    fn b0(&self) -> E;

    fn b1(&self) -> E;

    fn b2(&self) -> E;

    fn b3(&self) -> E;

    fn b4(&self) -> E;
}

impl<E: FieldElement> EvaluationFrameExtBits<E> for &EvaluationFrame<E> {
    fn b0(&self) -> E {
        self.current()[5]
    }

    fn b1(&self) -> E {
        self.current()[4]
    }

    fn b2(&self) -> E {
        self.current()[3]
    }

    fn b3(&self) -> E {
        self.current()[2]
    }

    fn b4(&self) -> E {
        self.current()[1]
    }
}

pub fn is_shr<E: FieldElement>(frame: &EvaluationFrame<E>) -> E {
    frame.b0()
}

pub fn is_shl<E: FieldElement>(frame: &EvaluationFrame<E>) -> E {
    frame.b1()
}

pub fn is_add<E: FieldElement>(frame: &EvaluationFrame<E>) -> E {
    not_(frame.b0()) * frame.b1() * not_(frame.b2()) * not_(frame.b3()) * not_(frame.b4())
}

pub fn is_sadd<E: FieldElement>(frame: &EvaluationFrame<E>) -> E {
    not_(frame.b0()) * frame.b1() * not_(frame.b2()) * frame.b3() * not_(frame.b4())
}

pub fn is_add2<E: FieldElement>(frame: &EvaluationFrame<E>) -> E {
    not_(frame.b0()) * frame.b1() * not_(frame.b2()) * frame.b3() * frame.b4()
}

pub fn is_mul<E: FieldElement>(frame: &EvaluationFrame<E>) -> E {
    not_(frame.b0()) * frame.b1() * not_(frame.b2()) * not_(frame.b3()) * frame.b4()
}

pub fn is_smul<E: FieldElement>(frame: &EvaluationFrame<E>) -> E {
    not_(frame.b0()) * frame.b1() * frame.b2() * not_(frame.b3()) * not_(frame.b4())
}

pub fn is_push<E: FieldElement>(frame: &EvaluationFrame<E>) -> E {
    frame.b0() * not_(frame.b1()) * not_(frame.b2()) * not_(frame.b3()) * not_(frame.b4())
}

pub fn is_read<E: FieldElement>(frame: &EvaluationFrame<E>) -> E {
    frame.b0() * not_(frame.b1()) * not_(frame.b2()) * not_(frame.b3()) * frame.b4()
}

pub fn is_read2<E: FieldElement>(frame: &EvaluationFrame<E>) -> E {
    frame.b0() * not_(frame.b1()) * not_(frame.b2()) * frame.b3() * not_(frame.b4())
}

pub fn is_noop<E: FieldElement>(frame: &EvaluationFrame<E>) -> E {
    not_(frame.b0()) * not_(frame.b1()) * not_(frame.b2()) * not_(frame.b3()) * not_(frame.b4())
}

pub fn opcode_to_element<E: FieldElement>(frame: &EvaluationFrame<E>) -> E {
    frame.b0() * E::from(16u8)
        + frame.b1() * E::from(8u8)
        + frame.b2() * E::from(4u8)
        + frame.b3() * E::from(2u8)
        + frame.b4()
}

pub fn not_<E: FieldElement>(bit: E) -> E {
    E::ONE - bit
}
