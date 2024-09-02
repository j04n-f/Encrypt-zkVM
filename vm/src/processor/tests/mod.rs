use fhe::{LweParameters, ServerKey};

use super::*;

#[cfg(test)]
mod stack;

#[test]
fn test_execute_program() {
    let source = "push.1 push.2 add read mul";
    let program = Program::load(source).unwrap();
    let processor = Processor::run(program, default_program_inputs()).unwrap();

    assert_eq!(processor.get_output()[0], 9);
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
