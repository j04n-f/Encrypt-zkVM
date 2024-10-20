// use fhe::{LweParameters, ServerKey};

use fhe::{LweParameters, ServerKey};

use super::*;

#[cfg(test)]
mod stack;

#[cfg(test)]
mod decoder;

#[cfg(test)]
mod system;

#[cfg(test)]
mod chiplets;

#[test]
fn test_trace() {
    let source = "push.5\npush.3\nadd";
    let program = Program::compile(source).unwrap();
    let hash = program.get_hash().to_elements().to_vec();
    let processor = Processor::run(program, program_inputs()).unwrap();
    let trace = processor.trace().unwrap();
    let trace_row31 = trace_state(31, &trace);

    assert_eq!(trace_row31[0], to_element(31));

    assert_eq!(trace_row31[1..6], to_elements(&[0, 0, 0, 0, 0]));

    assert_eq!(trace_row31[6], to_element(0));

    assert_eq!(trace_row31[7..9], hash);
    assert_eq!(trace_row31[9..11], [ZERO, ZERO]);

    assert_eq!(trace_row31[11], to_element(1));
    assert_eq!(trace_row31[12], to_element(8));
}

fn server_key() -> ServerKey {
    let plaintext_modulus: u32 = 8u32;
    let ciphertext_modulus: u32 = 128u32;
    let k: usize = 4;
    let std = 2.412_390_240_121_573e-5;
    let parameters = LweParameters::new(plaintext_modulus, ciphertext_modulus, k, std);

    ServerKey::new(parameters)
}

fn program_inputs() -> ProgramInputs {
    let server_key = server_key();

    let clear_x = 33u8;
    let clear_y = 7u8;

    let a = 3u8;
    let b = 12u8;
    let x = server_key.encrypt(clear_x);
    let y = server_key.encrypt(clear_y);

    ProgramInputs::new(&[a, b], &[x, y], &server_key)
}

fn empty_program_inputs() -> ProgramInputs {
    ProgramInputs::new(&[], &[], &server_key())
}

fn to_element(value: u8) -> BaseElement {
    BaseElement::from(value)
}

fn to_elements(arr: &[u8]) -> Vec<BaseElement> {
    arr.iter().map(|value| to_element(*value)).collect()
}

fn trace_state(clk: usize, trace: &[Vec<BaseElement>]) -> Vec<BaseElement> {
    let mut state = Vec::with_capacity(trace.len());
    for col in trace {
        state.push(col[clk]);
    }
    state
}
