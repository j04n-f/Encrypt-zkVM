use super::*;

use fhe::LweParameters;
use rescue::ARK;

use crate::flags::opcode_to_element;

use crate::constrains;

#[test]
fn test_enforce_clock_increase() {
    let mut current = Vec::from([BaseElement::ZERO; 28]);
    let mut next = Vec::from([BaseElement::ZERO; 28]);

    current[0] = BaseElement::from(3u8);
    next[0] = BaseElement::from(4u8);

    let frame = EvaluationFrame::<BaseElement>::from_rows(current, next);

    assert_eq!(constrains::enforce_clock_increase(&frame), BaseElement::ZERO)
}

#[test]
fn test_enforce_stack_shift() {
    for values in [[0u8, 0], [1, 0], [0, 1]].iter() {
        let mut current = Vec::from([BaseElement::ZERO; 28]);
        let next = Vec::from([BaseElement::ZERO; 28]);

        current[4] = BaseElement::from(values[0]);
        current[5] = BaseElement::from(values[1]);

        let frame = EvaluationFrame::<BaseElement>::from_rows(current, next);

        assert_eq!(constrains::enforce_stack_shift(&frame), BaseElement::ZERO);
    }
}

#[test]
fn test_enforce_stack_depth() {
    for (&depth, opcode) in
        [1i8, -1, 5, -5]
            .iter()
            .zip([[0u8, 0, 0, 0, 1], [0, 0, 0, 1, 0], [0, 1, 0, 0, 1], [1, 1, 0, 1, 0]])
    {
        let mut current = Vec::from([BaseElement::ZERO; 28]);
        let mut next = Vec::from([BaseElement::ZERO; 28]);

        current[1] = BaseElement::from(opcode[0]);
        current[2] = BaseElement::from(opcode[1]);
        current[3] = BaseElement::from(opcode[2]);
        current[4] = BaseElement::from(opcode[3]);
        current[5] = BaseElement::from(opcode[4]);

        current[11] = BaseElement::from(10u8);

        next[11] = BaseElement::from((10 + depth) as u8);

        let frame = EvaluationFrame::<BaseElement>::from_rows(current, next);

        assert_eq!(constrains::enforce_stack_depth(&frame), BaseElement::ZERO);
    }
}

#[test]
fn test_enforce_add() {
    let mut current = Vec::from([BaseElement::ZERO; 28]);
    let mut next = Vec::from([BaseElement::ZERO; 28]);

    current[4] = BaseElement::ONE;

    current[12] = BaseElement::from(4u8);
    current[13] = BaseElement::from(2u8);

    next[12] = BaseElement::from(6u8);

    let frame = EvaluationFrame::<BaseElement>::from_rows(current, next);

    assert_eq!(constrains::enforce_add(&frame), BaseElement::ZERO)
}

#[test]
fn test_enforce_sadd() {
    let mut current = Vec::from([BaseElement::ZERO; 28]);
    let mut next = Vec::from([BaseElement::ZERO; 28]);

    let server_key = server_key();

    let value = server_key.encrypt(4u8);
    let value_ct = value.ciphertext();

    current[2] = BaseElement::ONE;
    current[4] = BaseElement::ONE;

    current[12] = BaseElement::from(4u8);
    current[13] = value_ct[0];
    current[14] = value_ct[1];
    current[15] = value_ct[2];
    current[16] = value_ct[3];
    current[17] = value_ct[4];

    let result = server_key.scalar_add(&BaseElement::from(4u8), &value);

    let result_ct = result.ciphertext();

    next[12] = result_ct[0];
    next[13] = result_ct[1];
    next[14] = result_ct[2];
    next[15] = result_ct[3];
    next[16] = result_ct[4];

    let frame = EvaluationFrame::<BaseElement>::from_rows(current, next);

    assert_eq!(constrains::enforce_sadd(&frame, &server_key), BaseElement::ZERO)
}

#[test]
fn test_enforce_add2() {
    let mut current = Vec::from([BaseElement::ZERO; 28]);
    let mut next = Vec::from([BaseElement::ZERO; 28]);

    let server_key = server_key();

    let value0 = server_key.encrypt(4u8);
    let value_ct0 = value0.ciphertext();

    let value1 = server_key.encrypt(6u8);
    let value_ct1 = value1.ciphertext();

    current[1] = BaseElement::ONE;
    current[2] = BaseElement::ONE;
    current[4] = BaseElement::ONE;

    current[12] = value_ct0[0];
    current[13] = value_ct0[1];
    current[14] = value_ct0[2];
    current[15] = value_ct0[3];
    current[16] = value_ct0[4];

    current[17] = value_ct1[0];
    current[18] = value_ct1[1];
    current[19] = value_ct1[2];
    current[20] = value_ct1[3];
    current[21] = value_ct1[4];

    let result = server_key.add(&value0, &value1);

    let result_ct = result.ciphertext();

    next[12] = result_ct[0];
    next[13] = result_ct[1];
    next[14] = result_ct[2];
    next[15] = result_ct[3];
    next[16] = result_ct[4];

    let frame = EvaluationFrame::<BaseElement>::from_rows(current, next);

    assert_eq!(constrains::enforce_add2(&frame, &server_key), BaseElement::ZERO)
}

#[test]
fn test_enforce_mul() {
    let mut current = Vec::from([BaseElement::ZERO; 28]);
    let mut next = Vec::from([BaseElement::ZERO; 28]);

    current[1] = BaseElement::ONE;
    current[4] = BaseElement::ONE;

    current[12] = BaseElement::from(4u8);
    current[13] = BaseElement::from(2u8);

    next[12] = BaseElement::from(8u8);

    let frame = EvaluationFrame::<BaseElement>::from_rows(current, next);

    assert_eq!(constrains::enforce_mul(&frame), BaseElement::ZERO)
}

#[test]
fn test_enforce_smul() {
    let mut current = Vec::from([BaseElement::ZERO; 28]);
    let mut next = Vec::from([BaseElement::ZERO; 28]);

    let server_key = server_key();

    let value = server_key.encrypt(4u8);
    let value_ct = value.ciphertext();

    current[3] = BaseElement::ONE;
    current[4] = BaseElement::ONE;

    current[12] = BaseElement::from(4u8);
    current[13] = value_ct[0];
    current[14] = value_ct[1];
    current[15] = value_ct[2];
    current[16] = value_ct[3];
    current[17] = value_ct[4];

    let result = server_key.scalar_mul(&BaseElement::from(4u8), &value);

    let result_ct = result.ciphertext();

    next[12] = result_ct[0];
    next[13] = result_ct[1];
    next[14] = result_ct[2];
    next[15] = result_ct[3];
    next[16] = result_ct[4];

    let frame = EvaluationFrame::<BaseElement>::from_rows(current, next);

    assert_eq!(constrains::enforce_smul(&frame, &server_key), BaseElement::ZERO)
}

#[test]
fn test_enforce_push() {
    let mut current = Vec::from([BaseElement::ZERO; 28]);
    let mut next = Vec::from([BaseElement::ZERO; 28]);

    current[5] = BaseElement::ONE;

    current[12] = BaseElement::from(4u8);
    next[13] = BaseElement::from(4u8);

    let frame = EvaluationFrame::<BaseElement>::from_rows(current, next);

    assert_eq!(constrains::enforce_push(&frame), BaseElement::ZERO)
}

#[test]
fn test_enforce_read() {
    let mut current = Vec::from([BaseElement::ZERO; 28]);
    let mut next = Vec::from([BaseElement::ZERO; 28]);

    current[1] = BaseElement::ONE;
    current[5] = BaseElement::ONE;

    current[12] = BaseElement::from(4u8);
    next[13] = BaseElement::from(4u8);

    let frame = EvaluationFrame::<BaseElement>::from_rows(current, next);

    assert_eq!(constrains::enforce_read(&frame), BaseElement::ZERO)
}

#[test]
fn test_enforce_read2() {
    let mut current = Vec::from([BaseElement::ZERO; 28]);
    let mut next = Vec::from([BaseElement::ZERO; 28]);

    current[2] = BaseElement::ONE;
    current[5] = BaseElement::ONE;

    current[12] = BaseElement::from(4u8);
    next[17] = BaseElement::from(4u8);

    let frame = EvaluationFrame::<BaseElement>::from_rows(current, next);

    assert_eq!(constrains::enforce_read2(&frame), BaseElement::ZERO)
}

#[test]
fn test_enforce_noop() {
    let mut current = Vec::from([BaseElement::ZERO; 28]);
    let mut next = Vec::from([BaseElement::ZERO; 28]);

    current[12] = BaseElement::from(4u8);
    next[12] = BaseElement::from(4u8);

    let frame = EvaluationFrame::<BaseElement>::from_rows(current, next);

    assert_eq!(constrains::enforce_noop(&frame), BaseElement::ZERO)
}

#[test]
fn test_enforce_hash_round() {
    let mut current = Vec::from([BaseElement::ZERO; 28]);
    let mut next = Vec::from([BaseElement::ZERO; 28]);

    let mut state = [BaseElement::ZERO; 4];

    current[5] = BaseElement::ONE;

    current[6] = BaseElement::ONE;
    current[7] = state[0];
    current[8] = state[1];
    current[9] = state[2];
    current[10] = state[3];

    rescue::apply_round(&mut state, 16, 2, 0);

    next[7] = state[0];
    next[8] = state[1];
    next[9] = state[2];
    next[10] = state[3];

    next[12] = BaseElement::from(2u8);

    let frame = EvaluationFrame::<BaseElement>::from_rows(current, next);

    let mut result = [BaseElement::ONE; 4];

    constrains::enforce_hash_round(&frame, BaseElement::ONE, &ARK[0], &mut result);

    assert_eq!(result, [BaseElement::ZERO; 4])
}

#[test]
fn test_enforce_hash_copy() {
    let mut current = Vec::from([BaseElement::ZERO; 28]);
    let mut next = Vec::from([BaseElement::ZERO; 28]);

    current[6] = BaseElement::ONE;
    current[7] = BaseElement::from(2u8);
    current[8] = BaseElement::from(4u8);
    current[9] = BaseElement::from(6u8);
    current[10] = BaseElement::from(8u8);

    next[7] = BaseElement::from(2u8);
    next[8] = BaseElement::from(4u8);
    next[9] = BaseElement::ZERO;
    next[10] = BaseElement::ZERO;

    let frame = EvaluationFrame::<BaseElement>::from_rows(current, next);

    let mut result = [BaseElement::ONE; 4];

    constrains::enforce_hash_copy(&frame, BaseElement::ZERO, &mut result);

    assert_eq!(result, [BaseElement::ZERO; 4])
}

#[test]
fn test_opcode_to_element() {
    let mut current = Vec::from([BaseElement::ZERO; 6]);
    let next = Vec::from([BaseElement::ZERO; 6]);

    current[1] = BaseElement::ONE;
    current[2] = BaseElement::ONE;
    current[4] = BaseElement::ONE;

    let frame = EvaluationFrame::<BaseElement>::from_rows(current, next);

    assert_eq!(opcode_to_element(&frame), BaseElement::from(11u8))
}

fn server_key() -> ServerKey {
    let plaintext_modulus: u32 = 8u32;
    let ciphertext_modulus: u32 = 128u32;
    let k: usize = 4;
    let std = 2.412_390_240_121_573e-5;
    let parameters = LweParameters::new(plaintext_modulus, ciphertext_modulus, k, std);

    ServerKey::new(parameters)
}
