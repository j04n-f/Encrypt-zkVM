use winterfell::{math::fields::f128::BaseElement, FieldExtension, Proof, ProofOptions, Prover, TraceTable};

use prover::ExecutionProver;

mod program;
pub use program::{Program, ProgramInputs};

mod processor;
use processor::{Processor, StackError};

pub fn prove(program: Program, inputs: ProgramInputs) -> Result<(Vec<u128>, Proof), StackError> {
    let processor = Processor::run(program, inputs)?;

    let stack_output = processor.get_stack_output();

    let trace = TraceTable::init(
        processor
            .trace()
            .iter()
            .map(|trace| {
                trace
                    .iter()
                    .map(|value| BaseElement::try_from(*value).unwrap())
                    .collect::<Vec<BaseElement>>()
            })
            .collect::<Vec<Vec<BaseElement>>>(),
    );

    let options = ProofOptions::new(32, 8, 0, FieldExtension::None, 8, 127);

    let prover = ExecutionProver::new(
        options,
        stack_output
            .iter()
            .map(|value| BaseElement::try_from(*value).unwrap())
            .collect::<Vec<BaseElement>>(),
    );

    let proof = prover.prove(trace).unwrap();

    Ok((stack_output, proof))
}
