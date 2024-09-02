use std::{fs, io::Cursor};

use vm::{Processor, Program, ProgramInputs};

use fhe::{FheUInt8, LweParameters, ServerKey};

fn main() -> Result<(), String> {
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
    let mut inputs = Cursor::new(serialized_data);

    let public_inputs: [u8; 2] = bincode::deserialize_from(&mut inputs).unwrap();
    let secret_inputs: [FheUInt8; 1] = bincode::deserialize_from(&mut inputs).unwrap();
    let server_key: ServerKey = bincode::deserialize_from(&mut inputs).unwrap();

    let path = "lr.txt";

    let source = match fs::read_to_string(path) {
        Ok(source) => source,
        Err(err) => {
            return Err(format!(
                "could not open {}: {}",
                path,
                err.to_string().to_lowercase()
            ))
        }
    };

    let program = match Program::load(&source) {
        Ok(program) => program,
        Err(err) => return Err(format!("{err}")),
    };

    let processor = match Processor::run(
        program,
        ProgramInputs::new(&public_inputs, &secret_inputs, server_key),
    ) {
        Ok(output) => output,
        Err(err) => return Err(format!("{err}")),
    };

    let result = FheUInt8::new(&processor.get_output()[..5]);

    let mut serialized_result = Vec::new();
    bincode::serialize_into(&mut serialized_result, &result).unwrap();

    // Client
    let mut outputs = Cursor::new(serialized_result);
    let output: FheUInt8 = bincode::deserialize_from(&mut outputs).unwrap();

    let clear_result = client_key.decrypt(&output);

    assert_eq!(a * clear_x + b, clear_result);

    println!("Linear Regression: ({a} x {clear_x}) + {b} = {clear_result}");

    Ok(())
}
