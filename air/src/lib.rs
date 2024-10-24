mod constrains;
mod flags;

use fhe::ServerKey;
use winterfell::{
    math::{fields::f128::BaseElement, FieldElement, ToElements},
    Air, AirContext, Assertion, EvaluationFrame, ProofOptions, TraceInfo, TransitionConstraintDegree,
};

use crypto::{
    rescue,
    rescue::{CYCLE_LENGTH, DIGEST_SIZE},
};

#[cfg(test)]
mod tests;

pub struct PublicInputs {
    program_hash: [BaseElement; DIGEST_SIZE],
    stack_outputs: [BaseElement; 16],
    server_key: ServerKey,
}

impl PublicInputs {
    pub fn new(
        program_hash: [BaseElement; DIGEST_SIZE],
        stack_outputs: [BaseElement; 16],
        server_key: ServerKey,
    ) -> PublicInputs {
        PublicInputs {
            program_hash,
            stack_outputs,
            server_key,
        }
    }
}

impl ToElements<BaseElement> for PublicInputs {
    fn to_elements(&self) -> Vec<BaseElement> {
        let mut elements = Vec::new();

        elements.extend(&self.program_hash);
        elements.extend(&self.stack_outputs);

        elements
    }
}

pub struct ProcessorAir {
    context: AirContext<BaseElement>,
    program_hash: [BaseElement; DIGEST_SIZE],
    stack_outputs: [BaseElement; 16],
    server_key: ServerKey,
}

impl ProcessorAir {
    pub fn last_step(&self) -> usize {
        self.trace_length() - self.context().num_transition_exemptions()
    }
}

impl Air for ProcessorAir {
    type BaseField = BaseElement;
    type PublicInputs = PublicInputs;
    type GkrProof = ();
    type GkrVerifier = ();

    fn new(trace_info: TraceInfo, pub_inputs: PublicInputs, options: ProofOptions) -> Self {
        let degrees = vec![
            TransitionConstraintDegree::new(1),                             // clk
            TransitionConstraintDegree::new(5),                             // stack depth
            TransitionConstraintDegree::new(2),                             // shr or shl
            TransitionConstraintDegree::new(6),                             // add
            TransitionConstraintDegree::new(6),                             // sadd
            TransitionConstraintDegree::new(6),                             // add2
            TransitionConstraintDegree::new(7),                             // mul
            TransitionConstraintDegree::new(7),                             // smul
            TransitionConstraintDegree::new(6),                             // push
            TransitionConstraintDegree::new(6),                             // read
            TransitionConstraintDegree::new(6),                             // read2
            TransitionConstraintDegree::new(6),                             // noop
            TransitionConstraintDegree::with_cycles(4, vec![CYCLE_LENGTH]), // hash[0] round 0-14
            TransitionConstraintDegree::with_cycles(7, vec![CYCLE_LENGTH]), // hash[1] round 0-14
            TransitionConstraintDegree::with_cycles(4, vec![CYCLE_LENGTH]), // hash[2] round 0-14
            TransitionConstraintDegree::with_cycles(4, vec![CYCLE_LENGTH]), // hash[3] round 0-14
            TransitionConstraintDegree::with_cycles(2, vec![CYCLE_LENGTH]), // hash[0] round 14-16
            TransitionConstraintDegree::with_cycles(2, vec![CYCLE_LENGTH]), // hash[1] round 14-16
            TransitionConstraintDegree::with_cycles(2, vec![CYCLE_LENGTH]), // hash[2] round 14-16
            TransitionConstraintDegree::with_cycles(2, vec![CYCLE_LENGTH]), // hash[3] round 14-16
        ];

        // to improve the column degree computation
        // set transitions exemptions to allow random values on last row
        let air_context = AirContext::new(trace_info, degrees, 22, options).set_num_transition_exemptions(2);

        ProcessorAir {
            context: air_context,
            program_hash: pub_inputs.program_hash,
            stack_outputs: pub_inputs.stack_outputs,
            server_key: pub_inputs.server_key,
        }
    }

    fn evaluate_transition<E: FieldElement + From<Self::BaseField>>(
        &self,
        frame: &EvaluationFrame<E>,
        periodic_values: &[E],
        result: &mut [E],
    ) {
        // increase clk
        // clk' - (clk + 1) = 0 || deegre 1
        result[0] = constrains::enforce_clock_increase(frame);

        // increse or decrese stack depth by one or five
        // d' - d - flag_shr + flag_shr - (flag_read2 + flag_add2) * 4 = 0 || deegre 5
        result[1] = constrains::enforce_stack_depth(frame);

        // ensure shift is a binary operation
        // flag_shl * flag_shr = 0 || degree 2
        result[2] = constrains::enforce_stack_shift(frame);

        // add the two top stack elements
        // s0' - (s1 + s0) = 0 || degree 6
        result[3] = constrains::enforce_add(frame);

        // add the top scalar and ciphertext stack elements
        // s[0..5]' - (s0 + s[1..6]) = 0 || degree 6
        result[4] = constrains::enforce_sadd(frame, &self.server_key);

        // add the top ciphertext stack elements
        // s[0..5]' - (s0[0..5] + s[5..10]) = 0 || degree 6
        result[5] = constrains::enforce_add2(frame, &self.server_key);

        // multiply the two top stack elements
        // s0' - (s1 * s0) = 0 || degree 7
        result[6] = constrains::enforce_mul(frame);

        // multiply the top scalar and ciphertext stack elements
        // s[0..5]' - (s0 * s[1..6]) = 0 || degree 7
        result[7] = constrains::enforce_smul(frame, &self.server_key);

        // push a value the top of the stack
        // (s1' - s0) = 0 || degree 6
        // Pushed value onto the stack is injected (enforced) into sponge state
        result[8] = constrains::enforce_push(frame);

        // read an input and push to to the top of the stach
        // (s1' - s0) = 0 || degree 6
        result[9] = constrains::enforce_read(frame);

        // read2 a ciphertext and push to to the top of the stach
        // (s1' - s0) = 0 || degree 6
        result[10] = constrains::enforce_read2(frame);

        // copy the stack state
        // (s0' - s0) = 0 || degree 6
        result[11] = constrains::enforce_noop(frame);

        // Rescue-Prime
        let hash_flag = periodic_values[0];
        let ark = &periodic_values[1..];

        // apply hash round
        constrains::enforce_hash_round(frame, hash_flag, ark, &mut result[12..16]);

        // copy hash state and reset capacity values to 0
        constrains::enforce_hash_copy(frame, hash_flag, &mut result[16..20]);
    }

    fn get_assertions(&self) -> Vec<Assertion<Self::BaseField>> {
        let mut assertions = Vec::with_capacity(18);
        // initial clock value is 0
        assertions.push(Assertion::single(0, 0, Self::BaseField::ZERO));

        // initial stack depth is 0
        assertions.push(Assertion::single(11, 0, Self::BaseField::ZERO));

        let last_step = self.last_step();

        // initial hash state is 0
        // final hash state equals to program hash
        for i in 0..2 {
            assertions.push(Assertion::single(i + 7, 0, Self::BaseField::ZERO));
            assertions.push(Assertion::single(i + 7, last_step, self.program_hash[i]));
        }

        // initials stack state is 0
        // final stack state equals to output
        for i in 0..8 {
            assertions.push(Assertion::single(i + 12, 0, Self::BaseField::ZERO));
            assertions.push(Assertion::single(i + 12, last_step, self.stack_outputs[i]));
        }

        assertions
    }

    fn context(&self) -> &AirContext<Self::BaseField> {
        &self.context
    }

    fn get_periodic_column_values(&self) -> Vec<Vec<Self::BaseField>> {
        let mut result = vec![CYCLE_MASK.to_vec()];
        result.append(&mut rescue::get_round_constants());
        result
    }
}

const CYCLE_MASK: [BaseElement; CYCLE_LENGTH] = [
    BaseElement::ONE,
    BaseElement::ONE,
    BaseElement::ONE,
    BaseElement::ONE,
    BaseElement::ONE,
    BaseElement::ONE,
    BaseElement::ONE,
    BaseElement::ONE,
    BaseElement::ONE,
    BaseElement::ONE,
    BaseElement::ONE,
    BaseElement::ONE,
    BaseElement::ONE,
    BaseElement::ONE,
    BaseElement::ZERO,
    BaseElement::ZERO,
];
