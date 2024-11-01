use winterfell::{math::fields::f128::BaseElement, FieldExtension, Proof, ProofOptions, Prover, TraceTable};

use prover::ExecutionProver;

mod program;
pub use program::{Program, ProgramInputs};

mod processor;
use processor::{Processor, ProcessorError};

use crypto::rescue::Hash;

pub fn prove(program: Program, inputs: ProgramInputs) -> Result<(Hash, [BaseElement; 16], Proof), ProcessorError> {
    let processor = Processor::run(&program, &inputs)?;

    let output = processor.output();

    let trace = TraceTable::init(processor.trace()?);

    let options = ProofOptions::new(32, 8, 0, FieldExtension::None, 8, 127);

    let hash = program.hash();

    let prover = ExecutionProver::new(options, hash.to_elements(), output, inputs.server_key());

    let proof = prover.prove(trace).unwrap();

    Ok((hash, output, proof))
}

#[cfg(test)]
mod tests {
    use super::*;

    use air::{ProcessorAir, PublicInputs};
    use fhe::{FheUInt8, LweParameters, ServerKey};
    use std::io::Write;
    use tempfile::NamedTempFile;
    use winterfell::{
        crypto::{hashers::Blake3_256, DefaultRandomCoin},
        math::fields::f128::BaseElement,
        verify, AcceptableOptions,
    };

    type Blake3 = Blake3_256<BaseElement>;

    #[test]
    fn test_prove() {
        let mut tmpfile = NamedTempFile::new().unwrap();

        writeln!(tmpfile, "read2").unwrap();
        writeln!(tmpfile, "read").unwrap();
        writeln!(tmpfile, "sadd").unwrap();
        writeln!(tmpfile, "push.1").unwrap();
        writeln!(tmpfile, "push.2").unwrap();
        writeln!(tmpfile, "add").unwrap();
        writeln!(tmpfile, "smul").unwrap();

        let path = tmpfile.into_temp_path();

        let program = Program::load(&path).unwrap();

        let a = 1u8;
        let b = 3u8;

        let clear_x = 2u8;

        let plaintext_modulus: u32 = 8u32; // p
        let ciphertext_modulus: u32 = 128u32; // q
        let k: usize = 4; // This is the number of mask elements
        let std = 2.412_390_240_121_573e-5;
        let parameters = LweParameters::new(plaintext_modulus, ciphertext_modulus, k, std);

        let server_key = ServerKey::new(parameters);

        let x = server_key.encrypt(clear_x);

        let public_inputs = [a, b];
        let secret_inputs = [x];

        let inputs = ProgramInputs::new(&public_inputs, &secret_inputs, &server_key);

        let (hash, output, proof) = prove(program, inputs).unwrap();

        let result = FheUInt8::new(&output[..5]);

        let clear_result = server_key.decrypt(&result);

        assert_eq!((a + clear_x) * 3, clear_result);

        let min_opts = AcceptableOptions::MinConjecturedSecurity(95);

        verify::<ProcessorAir, Blake3, DefaultRandomCoin<Blake3>>(
            proof,
            PublicInputs::new(hash.to_elements(), output, server_key),
            &min_opts,
        )
        .unwrap()
    }
}
