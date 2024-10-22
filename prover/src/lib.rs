use fhe::ServerKey;
use winterfell::{
    crypto::{hashers::Blake3_256, DefaultRandomCoin},
    math::{fields::f128::BaseElement, FieldElement},
    matrix::ColMatrix,
    AuxRandElements, DefaultConstraintEvaluator, DefaultTraceLde, ProofOptions, Prover, StarkDomain, TraceInfo,
    TracePolyTable, TraceTable,
};

use air::{ProcessorAir, PublicInputs};

// We'll use BLAKE3 as the hash function during proof generation.
type Blake3 = Blake3_256<BaseElement>;

// Our prover needs to hold STARK protocol parameters which are specified via ProofOptions
// struct.
pub struct ExecutionProver {
    options: ProofOptions,
    program_hash: [BaseElement; 2],
    stack_outputs: [BaseElement; 16],
    server_key: ServerKey,
}

impl ExecutionProver {
    pub fn new(
        options: ProofOptions,
        program_hash: [BaseElement; 2],
        stack_outputs: [BaseElement; 16],
        server_key: ServerKey,
    ) -> Self {
        Self {
            options,
            stack_outputs,
            program_hash,
            server_key,
        }
    }
}

impl Prover for ExecutionProver {
    type BaseField = BaseElement;
    type Air = ProcessorAir;
    type Trace = TraceTable<BaseElement>;
    type HashFn = Blake3;
    type RandomCoin = DefaultRandomCoin<Blake3>;
    type TraceLde<E: FieldElement<BaseField = BaseElement>> = DefaultTraceLde<E, Blake3>;
    type ConstraintEvaluator<'a, E: FieldElement<BaseField = BaseElement>> =
        DefaultConstraintEvaluator<'a, ProcessorAir, E>;

    fn get_pub_inputs(&self, _trace: &Self::Trace) -> PublicInputs {
        PublicInputs::new(self.program_hash, self.stack_outputs, self.server_key.clone())
    }

    // We'll use the default trace low-degree extension.
    fn new_trace_lde<E: FieldElement<BaseField = Self::BaseField>>(
        &self,
        trace_info: &TraceInfo,
        main_trace: &ColMatrix<Self::BaseField>,
        domain: &StarkDomain<Self::BaseField>,
    ) -> (Self::TraceLde<E>, TracePolyTable<E>) {
        DefaultTraceLde::new(trace_info, main_trace, domain)
    }

    // We'll use the default constraint evaluator to evaluate AIR constraints.
    fn new_evaluator<'a, E: FieldElement<BaseField = BaseElement>>(
        &self,
        air: &'a ProcessorAir,
        aux_rand_elements: Option<AuxRandElements<E>>,
        composition_coefficients: winterfell::ConstraintCompositionCoefficients<E>,
    ) -> Self::ConstraintEvaluator<'a, E> {
        DefaultConstraintEvaluator::new(air, aux_rand_elements, composition_coefficients)
    }

    fn options(&self) -> &ProofOptions {
        &self.options
    }
}
