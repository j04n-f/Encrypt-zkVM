use std::{io::Cursor, path::Path};

use fhe::{FheUInt8, LweParameters, ServerKey};

use vm::{Program, ProgramInputs};

use winterfell::{
    crypto::{hashers::Blake3_256, DefaultRandomCoin},
    math::fields::f128::BaseElement,
    verify, AcceptableOptions, Proof,
};

use air::{ProcessAir, PublicInputs};

type Blake3 = Blake3_256<BaseElement>;

fn main() {
    let a = 2u8;
    let b = 12u8;

    let clear_x = 33u8;

    // Client
    let (serialized_data, client_key) = {
        let plaintext_modulus: u32 = 8u32; // p
        let ciphertext_modulus: u32 = 128u32; // q
        let k: usize = 4; // This is the number of mask elements
        let std = 2.412_390_240_121_573e-5;
        let parameters = LweParameters::new(plaintext_modulus, ciphertext_modulus, k, std);

        let client_key = ServerKey::new(parameters);

        let x = client_key.encrypt(clear_x);

        let mut serialized_data = Vec::new();
        bincode::serialize_into(&mut serialized_data, &[a, b]).unwrap();
        bincode::serialize_into(&mut serialized_data, &[x]).unwrap();
        bincode::serialize_into(&mut serialized_data, &client_key).unwrap();

        (serialized_data, client_key)
    };

    // Server
    let serialized_outputs = {
        let mut inputs = Cursor::new(serialized_data);

        let public_inputs: [u8; 2] = bincode::deserialize_from(&mut inputs).unwrap();
        let secret_inputs: [FheUInt8; 1] = bincode::deserialize_from(&mut inputs).unwrap();
        let server_key: ServerKey = bincode::deserialize_from(&mut inputs).unwrap();

        let path = Path::new("lr.txt");

        let program = Program::load(path).unwrap();

        let inputs = ProgramInputs::new(&public_inputs, &secret_inputs, server_key);

        let (output, proof) = vm::prove(program, inputs).unwrap();

        let result = FheUInt8::new(&output);

        let mut serialized_outputs = Vec::new();
        bincode::serialize_into(&mut serialized_outputs, &result).unwrap();
        bincode::serialize_into(&mut serialized_outputs, &proof.to_bytes()).unwrap();

        serialized_outputs
    };

    // Client
    let mut outputs = Cursor::new(serialized_outputs);
    let result: FheUInt8 = bincode::deserialize_from(&mut outputs).unwrap();
    let proof: Vec<u8> = bincode::deserialize_from(&mut outputs).unwrap();

    let clear_result = client_key.decrypt(&result);

    assert_eq!(a * clear_x + b, clear_result);

    let min_opts = AcceptableOptions::MinConjecturedSecurity(95);

    verify::<ProcessAir, Blake3, DefaultRandomCoin<Blake3>>(
        Proof::from_bytes(&proof).unwrap(),
        PublicInputs::new(),
        &min_opts,
    )
    .unwrap()
}
