use crypto::rescue::{self, STATE_WIDTH};
use fhe::{FheElement, ServerKey};
use winterfell::{
    math::{fields::f128::BaseElement, FieldElement},
    EvaluationFrame,
};

use crate::flags::{
    is_add, is_add2, is_mul, is_noop, is_push, is_read, is_read2, is_sadd, is_shl, is_shr, is_smul, not_,
    opcode_to_element,
};

trait EvaluationFrameExt<E: FieldElement> {
    fn stack_item(&self, index: usize) -> E;

    fn stack_items(&self, index: usize, size: usize) -> &[E];

    fn stack_item_next(&self, index: usize) -> E;

    fn stack_items_next(&self, index: usize, size: usize) -> &[E];

    fn hash(&self) -> &[E];

    fn hash_item(&self, index: usize) -> E;

    fn hash_next(&self) -> &[E];

    fn hash_item_next(&self, index: usize) -> E;

    fn stack_depth(&self) -> E;

    fn stack_depth_next(&self) -> E;

    fn clk(&self) -> E;

    fn clk_next(&self) -> E;

    fn h0(&self) -> E;
}

impl<E: FieldElement> EvaluationFrameExt<E> for &EvaluationFrame<E> {
    fn stack_item(&self, index: usize) -> E {
        self.current()[12 + index]
    }

    fn stack_items(&self, index: usize, size: usize) -> &[E] {
        &self.current()[(12 + index)..(12 + index + size)]
    }

    fn stack_item_next(&self, index: usize) -> E {
        self.next()[12 + index]
    }

    fn stack_items_next(&self, index: usize, size: usize) -> &[E] {
        &self.next()[(12 + index)..(12 + index + size)]
    }

    fn hash(&self) -> &[E] {
        &self.current()[7..11]
    }

    fn hash_item(&self, index: usize) -> E {
        self.current()[7 + index]
    }

    fn hash_next(&self) -> &[E] {
        &self.next()[7..11]
    }

    fn hash_item_next(&self, index: usize) -> E {
        self.next()[7 + index]
    }

    fn stack_depth(&self) -> E {
        self.current()[11]
    }

    fn stack_depth_next(&self) -> E {
        self.next()[11]
    }

    fn clk(&self) -> E {
        self.current()[0]
    }

    fn clk_next(&self) -> E {
        self.next()[0]
    }

    fn h0(&self) -> E {
        self.current()[6]
    }
}

pub fn enforce_clock_increase<E: FieldElement>(frame: &EvaluationFrame<E>) -> E {
    frame.clk_next() - (frame.clk() + E::ONE)
}

pub fn enforce_stack_shift<E: FieldElement>(frame: &EvaluationFrame<E>) -> E {
    is_shr(frame) * is_shl(frame)
}

pub fn enforce_stack_depth<E: FieldElement>(frame: &EvaluationFrame<E>) -> E {
    (frame.stack_depth_next() - frame.stack_depth() - is_shr(frame) + is_shl(frame)) - is_read2(frame) * E::from(4u8)
        + is_add2(frame) * E::from(4u8)
}

pub fn enforce_add<E: FieldElement>(frame: &EvaluationFrame<E>) -> E {
    is_add(frame) * (frame.stack_item_next(0) - (frame.stack_item(0) + frame.stack_item(1)))
}

pub fn enforce_sadd<E: FieldElement + From<BaseElement>>(frame: &EvaluationFrame<E>, server_key: &ServerKey) -> E {
    let stack_ct = frame.stack_items(1, server_key.lwe_size());
    let stack_ct_next = frame.stack_items_next(0, server_key.lwe_size());

    let value = FheElement::new(stack_ct);

    let output = server_key.scalar_add(&frame.stack_item(0), &value);

    is_sadd(frame)
        * stack_ct_next
            .iter()
            .zip(output.ciphertext().iter())
            .map(|(&a, &b)| a - b)
            .fold(E::ZERO, |acc, sum| acc + sum)
}

pub fn enforce_add2<E: FieldElement + From<BaseElement>>(frame: &EvaluationFrame<E>, server_key: &ServerKey) -> E {
    let stack_ct0 = frame.stack_items(0, server_key.lwe_size());
    let stack_ct1 = frame.stack_items(server_key.lwe_size(), server_key.lwe_size() * 2);
    let stack_ct_next = frame.stack_items_next(0, server_key.lwe_size());

    let value0 = FheElement::new(stack_ct0);
    let value1 = FheElement::new(stack_ct1);

    let output = server_key.add(&value0, &value1);

    is_add2(frame)
        * stack_ct_next
            .iter()
            .zip(output.ciphertext().iter())
            .map(|(&a, &b)| a - b)
            .fold(E::ZERO, |acc, sum| acc + sum)
}

pub fn enforce_mul<E: FieldElement>(frame: &EvaluationFrame<E>) -> E {
    is_mul(frame) * (frame.stack_item_next(0) - (frame.stack_item(0) * frame.stack_item(1)))
}

pub fn enforce_smul<E: FieldElement + From<BaseElement>>(frame: &EvaluationFrame<E>, server_key: &ServerKey) -> E {
    let stack_ct = frame.stack_items(1, server_key.lwe_size());
    let stack_ct_next = frame.stack_items_next(0, server_key.lwe_size());

    let value = FheElement::new(stack_ct);

    let output = server_key.scalar_mul(&frame.stack_item(0), &value);

    is_smul(frame)
        * stack_ct_next
            .iter()
            .zip(output.ciphertext().iter())
            .map(|(&a, &b)| a - b)
            .fold(E::ZERO, |acc, sum| acc + sum)
}

pub fn enforce_push<E: FieldElement>(frame: &EvaluationFrame<E>) -> E {
    is_push(frame) * (frame.stack_item_next(1) - frame.stack_item(0))
}

pub fn enforce_read<E: FieldElement>(frame: &EvaluationFrame<E>) -> E {
    is_read(frame) * (frame.stack_item_next(1) - frame.stack_item(0))
}

pub fn enforce_read2<E: FieldElement>(frame: &EvaluationFrame<E>) -> E {
    is_read2(frame) * (frame.stack_item_next(5) - frame.stack_item(0))
}

pub fn enforce_noop<E: FieldElement>(frame: &EvaluationFrame<E>) -> E {
    is_noop(frame) * (frame.stack_item_next(0) - frame.stack_item(0))
}

pub fn enforce_hash_round<E: FieldElement + From<BaseElement>>(
    frame: &EvaluationFrame<E>,
    hash_flag: E,
    ark: &[E],
    result: &mut [E],
) {
    let mut step0 = frame.hash().to_vec();
    rescue::apply_sbox(&mut step0);
    rescue::apply_mds(&mut step0);
    for i in 0..STATE_WIDTH {
        step0[i] += ark[i];
    }

    step0[0] += opcode_to_element(frame);
    step0[1] += frame.stack_item_next(0) * is_push(frame);

    let mut step1 = frame.hash_next().to_vec();
    for i in 0..STATE_WIDTH {
        step1[i] -= ark[STATE_WIDTH + i];
    }
    rescue::apply_inv_mds(&mut step1);
    rescue::apply_sbox(&mut step1);

    result[0] = (step1[0] - step0[0]) * hash_flag * frame.h0();
    result[1] = (step1[1] - step0[1]) * hash_flag * frame.h0();
    result[2] = (step1[2] - step0[2]) * hash_flag * frame.h0();
    result[3] = (step1[3] - step0[3]) * hash_flag * frame.h0();
}

pub fn enforce_hash_copy<E: FieldElement>(frame: &EvaluationFrame<E>, hash_flag: E, result: &mut [E]) {
    result[0] = (frame.hash_item_next(0) - frame.hash_item(0)) * not_(hash_flag) * frame.h0();
    result[1] = (frame.hash_item_next(1) - frame.hash_item(1)) * not_(hash_flag) * frame.h0();
    result[2] = frame.hash_item_next(2) * not_(hash_flag) * frame.h0();
    result[3] = frame.hash_item_next(3) * not_(hash_flag) * frame.h0();
}
