use winterfell::{
    math::{fields::f128::BaseElement, FieldElement, ToElements},
    Air, AirContext, Assertion, EvaluationFrame, ProofOptions, TraceInfo,
    TransitionConstraintDegree,
};

pub struct PublicInputs();

impl Default for PublicInputs {
    fn default() -> Self {
        Self::new()
    }
}

impl PublicInputs {
    pub fn new() -> PublicInputs {
        PublicInputs {}
    }
}

impl ToElements<BaseElement> for PublicInputs {
    fn to_elements(&self) -> Vec<BaseElement> {
        vec![]
    }
}

pub struct ProcessAir {
    context: AirContext<BaseElement>,
}

impl Air for ProcessAir {
    type BaseField = BaseElement;
    type PublicInputs = PublicInputs;
    type GkrProof = ();
    type GkrVerifier = ();

    fn new(trace_info: TraceInfo, _pub_inputs: PublicInputs, options: ProofOptions) -> Self {
        // 0 = clk' - (clk + 1) || degree 1
        let degrees = vec![TransitionConstraintDegree::new(1)];

        ProcessAir {
            context: AirContext::new(trace_info, degrees, 1, options),
        }
    }

    fn evaluate_transition<E: FieldElement + From<Self::BaseField>>(
        &self,
        frame: &EvaluationFrame<E>,
        _periodic_values: &[E],
        result: &mut [E],
    ) {
        // 0 = clk' - (clk + 1)
        result[0] = frame.next()[0] - (frame.current()[0] + E::ONE);
    }

    fn get_assertions(&self) -> Vec<Assertion<Self::BaseField>> {
        vec![Assertion::single(0, 0, Self::BaseField::ZERO)]
    }

    fn context(&self) -> &AirContext<Self::BaseField> {
        &self.context
    }
}
