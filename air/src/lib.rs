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
        let degrees = vec![
            TransitionConstraintDegree::new(1),
            TransitionConstraintDegree::new(2),
            TransitionConstraintDegree::new(2),
            TransitionConstraintDegree::new(4),
            TransitionConstraintDegree::new(5),
            TransitionConstraintDegree::new(4),
            TransitionConstraintDegree::new(4),
            TransitionConstraintDegree::new(2),
        ];

        ProcessAir {
            context: AirContext::new(trace_info, degrees, 1, options)
                .set_num_transition_exemptions(2),
        }
    }

    fn evaluate_transition<E: FieldElement + From<Self::BaseField>>(
        &self,
        frame: &EvaluationFrame<E>,
        _periodic_values: &[E],
        result: &mut [E],
    ) {
        let b0 = frame.current()[3];
        let b1 = frame.current()[2];
        let b2 = frame.current()[1];

        // Increase clk
        // 0 = clk' - (clk + 1) || deegre 1
        result[0] = frame.next()[0] - (frame.current()[0] + E::ONE);

        // Stack Depth
        result[1] = frame.next()[4] - frame.current()[4] - b1 + b0 * (E::ONE - b1);

        // Shr o Shl
        result[2] = b0 * (E::ONE - b0);

        // Add
        // 0 = b0 * (1 - b1) * b2 * (s0' - (s0 + s1)) || degree 4
        result[3] =
            b0 * (E::ONE - b1) * b2 * (frame.next()[5] - (frame.current()[5] + frame.current()[6]));

        // Mul
        // 0 = b0 * (1 - b1) * (1 - b2) * (s0' - s0 * s1) || degree 5
        result[4] = b0
            * (E::ONE - b1)
            * (E::ONE - b2)
            * (frame.next()[5] - frame.current()[5] * frame.current()[6]);

        // Push
        // 0 = b0 * b1 * (1 - b2) * (s1' - s0) || degree 4
        // Pushed value onto the stack is injected (enforced) into sponge state
        result[5] = b0 * b1 * (E::ONE - b2) * (frame.next()[6] - frame.current()[5]);

        // Read
        // 0 = b0 * b1 * b2 * (s1' - s0) || degree 4
        // Read value onto the stack is injected (enforced) into sponge state
        result[6] = b0 * b1 * b2 * (frame.next()[6] - frame.current()[5]);

        // Noop
        // 0 = (1 - b2) * (1 - b2) * (1 - b2) * (s1' - s0) || degree 2
        result[7] = (E::ONE - b0) * (frame.next()[5] - frame.current()[5]);
    }

    fn get_assertions(&self) -> Vec<Assertion<Self::BaseField>> {
        // clk[0] = 0
        vec![Assertion::single(0, 0, Self::BaseField::ZERO)]
    }

    fn context(&self) -> &AirContext<Self::BaseField> {
        &self.context
    }
}
