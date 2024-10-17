use fhe::{FheElement, ServerKey};
use winterfell::{
    math::{fields::f128::BaseElement, FieldElement, ToElements},
    Air, AirContext, Assertion, EvaluationFrame, ProofOptions, TraceInfo, TransitionConstraintDegree,
};

use crypto::{
    rescue,
    rescue::{CYCLE_LENGTH, DIGEST_SIZE, STATE_WIDTH},
};

pub struct PublicInputs {
    program_hash: [BaseElement; DIGEST_SIZE],
    stack_outputs: Vec<BaseElement>,
    server_key: ServerKey,
}

impl PublicInputs {
    pub fn new(
        program_hash: [BaseElement; DIGEST_SIZE],
        stack_outputs: Vec<BaseElement>,
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
    stack_outputs: Vec<BaseElement>,
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
            TransitionConstraintDegree::new(1),
            TransitionConstraintDegree::new(5),
            TransitionConstraintDegree::new(2),
            TransitionConstraintDegree::new(6),
            TransitionConstraintDegree::new(6),
            TransitionConstraintDegree::new(7),
            TransitionConstraintDegree::new(7),
            TransitionConstraintDegree::new(6),
            TransitionConstraintDegree::new(6),
            TransitionConstraintDegree::new(6),
            TransitionConstraintDegree::new(6),
            TransitionConstraintDegree::with_cycles(4, vec![CYCLE_LENGTH]),
            TransitionConstraintDegree::with_cycles(7, vec![CYCLE_LENGTH]),
            TransitionConstraintDegree::with_cycles(4, vec![CYCLE_LENGTH]),
            TransitionConstraintDegree::with_cycles(4, vec![CYCLE_LENGTH]),
            TransitionConstraintDegree::with_cycles(2, vec![CYCLE_LENGTH]),
            TransitionConstraintDegree::with_cycles(2, vec![CYCLE_LENGTH]),
            TransitionConstraintDegree::with_cycles(2, vec![CYCLE_LENGTH]),
            TransitionConstraintDegree::with_cycles(2, vec![CYCLE_LENGTH]),
        ];

        // allow to transition exemptions
        // last row has random values
        // to improve the column degree computation
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
        let b0 = frame.current()[5]; // Shr
        let b1 = frame.current()[4]; // Shl
        let b2 = frame.current()[3]; // Op
        let b3 = frame.current()[2]; // Op
        let b4 = frame.current()[1]; // Op

        // Increase clk
        // 0 = clk' - (clk + 1) || deegre 1
        result[0] = frame.next()[0] - (frame.current()[0] + E::ONE);

        // Stack Depth || deegre 5
        result[1] = (frame.next()[11] - frame.current()[11] - b0 + b1)
            - b0 * not_(b1) * not_(b2) * b3 * not_(b4) * E::from(4u8);

        // Shr or Shl || deegre 2
        result[2] = b0 * b1;

        // Add
        // 0 = b0 * (1 - b1) * (1 - b2) * b3 * (s0' - (s0 + s1)) || degree 6
        result[3] = not_(b0)
            * b1
            * not_(b2)
            * not_(b3)
            * not_(b4)
            * (frame.next()[12] - (frame.current()[12] + frame.current()[13]));

        // SAdd
        let value = FheElement::new(&frame.current()[13..18]);
        let output = self.server_key.scalar_add(&frame.current()[12], &value);
        let ct = output.ciphertext();

        result[4] = not_(b0)
            * b1
            * not_(b2)
            * b3
            * not_(b4)
            * (frame.next()[12] + frame.next()[13] + frame.next()[14] + frame.next()[15] + frame.next()[16]
                - ct[0]
                - ct[1]
                - ct[2]
                - ct[3]
                - ct[4]);

        // Mul
        // 0 = b0 * (1 - b1) * (1 - b2) * (1 - b3) * (s0' - s0 * s1) || degree 7
        result[5] =
            not_(b0) * b1 * not_(b2) * not_(b3) * b4 * (frame.next()[12] - frame.current()[12] * frame.current()[13]);

        // SMul
        let value = FheElement::new(&frame.current()[13..18]);
        let output = self.server_key.scalar_mul(&frame.current()[12], &value);
        let ct = output.ciphertext();

        result[6] = not_(b0)
            * b1
            * b2
            * not_(b3)
            * not_(b4)
            * (frame.next()[12] + frame.next()[13] + frame.next()[14] + frame.next()[15] + frame.next()[16]
                - ct[0]
                - ct[1]
                - ct[2]
                - ct[3]
                - ct[4]);

        // Push
        // 0 = b0 * (1 - b1) * b2 * (1 - b3) * (s1' - s0) || degree 6
        // Pushed value onto the stack is injected (enforced) into sponge state
        result[7] = b0 * not_(b1) * not_(b2) * not_(b3) * not_(b4) * (frame.next()[13] - frame.current()[12]);

        // Read
        // 0 = b0 * (1 - b1) * b2 * b3 * (s1' - s0) || degree 6
        result[8] = b0 * not_(b1) * not_(b2) * not_(b3) * b4 * (frame.next()[13] - frame.current()[12]);

        // Read2
        // 0 = b0 * b1 * b2 * b3 * (s1' - s0) || degree 6
        result[9] = b0 * not_(b1) * not_(b2) * b3 * not_(b4) * (frame.next()[17] - frame.current()[12]);

        // Noop
        // 0 = (1 - b0) * (s0' - s0) || degree 6
        result[10] = not_(b0) * not_(b1) * not_(b2) * not_(b3) * not_(b4) * (frame.next()[12] - frame.current()[12]);

        // Rescue-Prime
        let hash_flag = periodic_values[0];
        let ark = &periodic_values[1..];

        let push_flag = b0 * not_(b1) * not_(b2) * not_(b3) * not_(b4);

        let mut step0 = frame.current()[7..11].to_vec();
        rescue::apply_sbox(&mut step0);
        rescue::apply_mds(&mut step0);
        for i in 0..STATE_WIDTH {
            step0[i] += ark[i];
        }

        step0[0] += b0 * E::from(16u8) + b1 * E::from(8u8) + b2 * E::from(4u8) + b3 * E::from(2u8) + b4;
        step0[1] += frame.next()[12] * push_flag;

        let mut step1 = frame.next()[7..11].to_vec();
        for i in 0..STATE_WIDTH {
            step1[i] -= ark[STATE_WIDTH + i];
        }
        rescue::apply_inv_mds(&mut step1);
        rescue::apply_sbox(&mut step1);

        result[11] = (step1[0] - step0[0]) * hash_flag * frame.current()[6];
        result[12] = (step1[1] - step0[1]) * hash_flag * frame.current()[6];
        result[13] = (step1[2] - step0[2]) * hash_flag * frame.current()[6];
        result[14] = (step1[3] - step0[3]) * hash_flag * frame.current()[6];

        result[15] = (frame.next()[7] - frame.current()[7]) * not_(hash_flag) * frame.current()[6];
        result[16] = (frame.next()[8] - frame.current()[8]) * not_(hash_flag) * frame.current()[6];
        result[17] = frame.next()[9] * not_(hash_flag) * frame.current()[6];
        result[18] = frame.next()[10] * not_(hash_flag) * frame.current()[6];
    }

    fn get_assertions(&self) -> Vec<Assertion<Self::BaseField>> {
        let mut assertions = Vec::with_capacity(18);
        // clk[0] = 0
        assertions.push(Assertion::single(0, 0, Self::BaseField::ZERO));

        // depth[0] = 0
        assertions.push(Assertion::single(11, 0, Self::BaseField::ZERO));

        let last_step = self.last_step();

        // Initial Hash == 0
        // Final Hash == Program Hash
        for i in 0..2 {
            assertions.push(Assertion::single(i + 7, 0, Self::BaseField::ZERO));
            assertions.push(Assertion::single(i + 7, last_step, self.program_hash[i]));
        }

        // Initial Stack == 0
        // Final Stack == Stack Output
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

fn not_<E: FieldElement + From<<ProcessorAir as Air>::BaseField>>(bit: E) -> E {
    E::ONE - bit
}
