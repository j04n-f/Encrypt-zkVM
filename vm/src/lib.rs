use winterfell::{math::fields::f128::BaseElement, FieldExtension, Proof, ProofOptions, Prover, TraceTable};

use prover::ExecutionProver;

mod program;
pub use program::{Program, ProgramInputs};

mod processor;
use processor::{Processor, ProcessorError};

use crypto::rescue::Hash;

pub fn prove(program: Program, inputs: ProgramInputs) -> Result<(Hash, Vec<BaseElement>, Proof), ProcessorError> {
    let program_hash = program.get_hash();
    let secret_key = inputs.get_server_key();
    let processor = Processor::run(program, inputs)?;

    let stack_output = processor.get_stack_output();

    let trace = TraceTable::init(processor.trace()?);

    let options = ProofOptions::new(32, 8, 0, FieldExtension::None, 8, 127);

    let prover = ExecutionProver::new(options, program_hash.to_elements(), &stack_output, secret_key);

    let proof = prover.prove(trace).unwrap();

    Ok((program_hash, stack_output, proof))
}
