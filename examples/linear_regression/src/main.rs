use std::path::Path;

use fhe::{FheUInt8, LweParameters, ServerKey};

use vm::{Program, ProgramInputs};

use winterfell::{
    crypto::{hashers::Blake3_256, DefaultRandomCoin},
    math::fields::f128::BaseElement,
    verify, AcceptableOptions, Deserializable, Serializable,
};

use air::{ProcessorAir, PublicInputs};

mod utils;
use utils::{InputData, OutputData};

type Blake3 = Blake3_256<BaseElement>;

fn main() {
    let b0 = 1u8;
    let b1 = 3u8;
    let b2 = 2u8;
    let b3 = 4u8;
    let b4 = 2u8;

    let clear_x1 = 2u8;
    let clear_x2 = 3u8;
    let clear_x3 = 3u8;
    let clear_x4 = 2u8;

    // Client
    let (input_data, client_key) = {
        let plaintext_modulus: u32 = 8u32; // p
        let ciphertext_modulus: u32 = 128u32; // q
        let k: usize = 4; // This is the number of mask elements
        let std = 2.412_390_240_121_573e-5;
        let parameters = LweParameters::new(plaintext_modulus, ciphertext_modulus, k, std);

        let client_key = ServerKey::new(parameters);

        let x1 = client_key.encrypt(clear_x1);
        let x2 = client_key.encrypt(clear_x2);
        let x3 = client_key.encrypt(clear_x3);
        let x4 = client_key.encrypt(clear_x4);

        let data = InputData::new(&[b0, b1, b2, b3, b4], &[x1, x2, x3, x4], &client_key);

        (data.to_bytes(), client_key)
    };

    // Server
    let output_data = {
        let payload = InputData::read_from_bytes(&input_data).unwrap();

        let path = Path::new("lr.txt");

        let program = Program::load(path).unwrap();

        let inputs = ProgramInputs::new(payload.public_inputs(), payload.secret_inputs(), payload.server_key());

        let (hash, output, proof) = vm::prove(program, inputs).unwrap();

        let output = OutputData::new(hash, proof, &output);

        output.to_bytes()
    };

    // Client
    let results = OutputData::read_from_bytes(&output_data).unwrap();

    let result = FheUInt8::new(&results.output()[..5]);

    let clear_result = client_key.decrypt(&result);

    assert_eq!(
        b0 + b1 * clear_x1 + b2 * clear_x2 + b3 * clear_x3 + b4 * clear_x4,
        clear_result
    );

    let min_opts = AcceptableOptions::MinConjecturedSecurity(95);

    verify::<ProcessorAir, Blake3, DefaultRandomCoin<Blake3>>(
        results.proof().clone(),
        PublicInputs::new(results.hash().to_elements(), results.output().to_vec(), client_key),
        &min_opts,
    )
    .unwrap()
}
