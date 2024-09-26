use fhe::{LweParameters, ServerKey};

use super::*;

#[cfg(test)]
mod stack;

#[cfg(test)]
mod decoder;

#[cfg(test)]
mod system;

#[test]
fn test_execute_program() {
    let source = "push.1 push.2 add read mul";
    let program = Program::compile(source).unwrap();
    let processor = Processor::run(program, default_program_inputs()).unwrap();

    assert_eq!(processor.get_stack_output()[0], 9);
}

#[test]
fn test_execution_trace() {
    let source = "push.1 push.2 add read mul";
    let program = Program::compile(source).unwrap();
    let processor: Processor = Processor::run(program, default_program_inputs()).unwrap();

    let trace = processor.trace();

    assert_eq!(trace_state(0, &trace), vec![0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    assert_eq!(trace_state(1, &trace), vec![1, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0]);
    assert_eq!(trace_state(2, &trace), vec![2, 1, 0, 1, 2, 2, 1, 0, 0, 0, 0, 0, 0]);
    assert_eq!(trace_state(3, &trace), vec![3, 1, 1, 1, 1, 3, 0, 0, 0, 0, 0, 0, 0]);
    assert_eq!(trace_state(4, &trace), vec![4, 0, 0, 1, 2, 3, 3, 0, 0, 0, 0, 0, 0]);
    assert_eq!(trace_state(5, &trace), vec![5, 0, 0, 0, 1, 9, 0, 0, 0, 0, 0, 0, 0]);


    for i in 6..15 {
        assert_eq!(trace_state(i, &trace), vec![i as u128, 0, 0, 0, 1, 9, 0, 0, 0, 0, 0, 0, 0]);
    }

    for col in trace_state(15, &trace).iter() {
        assert!(*col != 0);
    }
}

fn default_program_inputs() -> ProgramInputs {
    let plaintext_modulus: u32 = 8u32;
    let ciphertext_modulus: u32 = 128u32;
    let k: usize = 4;
    let std = 2.412_390_240_121_573e-5;
    let parameters = LweParameters::new(plaintext_modulus, ciphertext_modulus, k, std);

    let server_key = ServerKey::new(parameters);

    let clear_x = 33u8;

    let a = 3u8;
    let b = 12u8;
    let x = server_key.encrypt(clear_x);

    ProgramInputs::new(&[a, b], &[x], server_key)
}

pub fn trace_state(clk: usize, trace: &[Vec<u128>]) -> Vec<u128> {
    let mut state = Vec::with_capacity(trace.len());
    for col in trace {
        state.push(col[clk]);
    }
    state
}
