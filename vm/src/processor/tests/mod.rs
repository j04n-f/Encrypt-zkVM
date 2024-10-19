// use fhe::{LweParameters, ServerKey};

use fhe::{LweParameters, ServerKey};

use super::*;

#[cfg(test)]
mod stack;

#[cfg(test)]
mod decoder;

#[cfg(test)]
mod system;

// mod push {

//     use super::*;

//     #[test]
//     fn test_trace() {
//         let source = "push.5";
//         let program = Program::compile(source).unwrap();
//         let processor = Processor::run(program, program_inputs()).unwrap();
//         let trace = processor.trace();
//         let trace_row0 = trace_state(0, &trace);
//         let trace_row1 = trace_state(1, &trace);

//         let mut state = vec![ZERO; 4];
//         let op = Operation::push(5);

//         rescue::apply_round(&mut state, op.code(), op.value(), 0);

//         assert_eq!(trace_row0[0], to_element(0));
//         assert_eq!(trace_row1[0], to_element(1));

//         assert_eq!(trace_row0[1..6], to_elements(&[0, 0, 0, 0, 1]));

//         assert_eq!(trace_row0[6], to_element(1));

//         assert_eq!(trace_row1[7..11], state);

//         assert_eq!(trace_row0[11], to_element(0));
//         assert_eq!(trace_row1[11], to_element(1));

//         assert_eq!(trace_row0[12], to_element(0));
//         assert_eq!(trace_row1[12], to_element(5));
//     }
// }

// mod read {

//     use super::*;

//     #[test]
//     fn test_trace() {
//         let source = "read";
//         let program = Program::compile(source).unwrap();
//         let processor = Processor::run(program, program_inputs()).unwrap();
//         let trace = processor.trace();
//         let trace_row0 = trace_state(0, &trace);
//         let trace_row1 = trace_state(1, &trace);

//         let mut state = vec![ZERO; 4];
//         let op = Operation::read();

//         rescue::apply_round(&mut state, op.code(), op.value(), 0);

//         assert_eq!(trace_row0[0], to_element(0));
//         assert_eq!(trace_row1[0], to_element(1));

//         assert_eq!(trace_row0[1..6], to_elements(&[1, 0, 0, 0, 1]));

//         assert_eq!(trace_row0[6], to_element(1));

//         assert_eq!(trace_row1[7..11], state);

//         assert_eq!(trace_row0[11], to_element(0));
//         assert_eq!(trace_row1[11], to_element(1));

//         assert_eq!(trace_row0[12], to_element(0));
//         assert_eq!(trace_row1[12], to_element(3));
//     }

//     #[test]
//     fn test_empty_inputs() {
//         let source = "read";
//         let program = Program::compile(source).unwrap();
//         let error = Processor::run(program, empty_program_inputs()).unwrap_err();

//         assert_eq!(
//             format!("{error}"),
//             format!("{}", StackError::empty_inputs(1))
//         );
//     }
// }

// mod read2 {

//     use super::*;

//     #[test]
//     fn test_trace() {
//         let source = "read2";
//         let program = Program::compile(source).unwrap();
//         let inputs = program_inputs();
//         let input_ct = inputs.get_secret()[0].ciphertext().to_vec();
//         let processor = Processor::run(program, inputs).unwrap();
//         let trace = processor.trace();
//         let trace_row0 = trace_state(0, &trace);
//         let trace_row1 = trace_state(1, &trace);

//         let mut state = vec![ZERO; 4];
//         let op = Operation::read2();

//         rescue::apply_round(&mut state, op.code(), op.value(), 0);

//         assert_eq!(trace_row0[0], to_element(0));
//         assert_eq!(trace_row1[0], to_element(1));

//         assert_eq!(trace_row0[1..6], to_elements(&[0, 1, 0, 0, 1]));

//         assert_eq!(trace_row0[6], to_element(1));

//         assert_eq!(trace_row1[7..11], state);

//         assert_eq!(trace_row0[11], to_element(0));
//         assert_eq!(trace_row1[11], to_element(5));

//         assert_eq!(trace_row0[12..17], to_elements(&[0, 0, 0, 0, 0]));
//         assert_eq!(trace_row1[12..17], input_ct);
//     }

//     #[test]
//     fn test_empty_inputs() {
//         let source = "read2";
//         let program = Program::compile(source).unwrap();
//         let error = Processor::run(program, empty_program_inputs()).unwrap_err();

//         assert_eq!(
//             format!("{error}"),
//             format!("{}", StackError::empty_inputs(1))
//         );
//     }
// }

// mod add {

//     use crypto::Rescue128;

//     use super::*;

//     #[test]
//     fn test_trace() {
//         let source = "push.1\npush.2\nadd";
//         let program = Program::compile(source).unwrap();
//         let code = program.get_code().to_vec();
//         let inputs = program_inputs();
//         let processor = Processor::run(program, inputs).unwrap();
//         let trace = processor.trace();
//         let trace_row9 = trace_state(9, &trace);
//         let trace_row10 = trace_state(10, &trace);

//         let mut sponge = Rescue128::new();

//         for op in code[0..10].iter() {
//             sponge.update(op.code(), op.value());
//         }

//         assert_eq!(trace_row9[0], to_element(9));
//         assert_eq!(trace_row10[0], to_element(10));

//         assert_eq!(trace_row9[1..6], to_elements(&[0, 0, 0, 1, 0]));

//         assert_eq!(trace_row9[6], to_element(1));

//         assert_eq!(trace_row10[7..11], sponge.state());

//         assert_eq!(trace_row9[11], to_element(2));
//         assert_eq!(trace_row10[11], to_element(1));

//         assert_eq!(trace_row10[12], to_element(3));
//     }

//     #[test]
//     fn test_stack_underflow() {
//         let source = "push.1\nadd";
//         let program = Program::compile(source).unwrap();
//         let error = Processor::run(program, empty_program_inputs()).unwrap_err();

//         assert_eq!(
//             format!("{error}"),
//             format!("{}", StackError::stack_underflow("add", 2))
//         );
//     }
// }

// mod mul {

//     use crypto::Rescue128;

//     use super::*;

//     #[test]
//     fn test_trace() {
//         let source = "push.1\npush.2\nmul";
//         let program = Program::compile(source).unwrap();
//         let code = program.get_code().to_vec();
//         let inputs = program_inputs();
//         let processor = Processor::run(program, inputs).unwrap();
//         let trace = processor.trace();
//         let trace_row9 = trace_state(9, &trace);
//         let trace_row10 = trace_state(10, &trace);

//         let mut sponge = Rescue128::new();

//         for op in code[0..10].iter() {
//             sponge.update(op.code(), op.value());
//         }

//         assert_eq!(trace_row9[0], to_element(9));
//         assert_eq!(trace_row10[0], to_element(10));

//         assert_eq!(trace_row9[1..6], to_elements(&[1, 0, 0, 1, 0]));

//         assert_eq!(trace_row9[6], to_element(1));

//         assert_eq!(trace_row10[7..11], sponge.state());

//         assert_eq!(trace_row9[11], to_element(2));
//         assert_eq!(trace_row10[11], to_element(1));

//         assert_eq!(trace_row10[12], to_element(2));
//     }

//     #[test]
//     fn test_stack_underflow() {
//         let source = "push.1\nmul";
//         let program = Program::compile(source).unwrap();
//         let error = Processor::run(program, empty_program_inputs()).unwrap_err();

//         assert_eq!(
//             format!("{error}"),
//             format!("{}", StackError::stack_underflow("mul", 2))
//         );
//     }
// }

// mod sadd {

//     use crypto::Rescue128;

//     use super::*;

//     #[test]
//     fn test_trace() {
//         let source = "read2\nread\nsadd";
//         let program = Program::compile(source).unwrap();
//         let code = program.get_code().to_vec();
//         let inputs = program_inputs();
//         let processor = Processor::run(program, inputs).unwrap();
//         let trace = processor.trace();
//         let trace_row9 = trace_state(9, &trace);
//         let trace_row10 = trace_state(10, &trace);

//         let mut sponge = Rescue128::new();

//         for op in code[0..10].iter() {
//             sponge.update(op.code(), op.value());
//         }

//         assert_eq!(trace_row9[0], to_element(9));
//         assert_eq!(trace_row10[0], to_element(10));

//         assert_eq!(trace_row9[1..6], to_elements(&[1, 0, 0, 1, 0]));

//         assert_eq!(trace_row9[6], to_element(1));

//         assert_eq!(trace_row10[7..11], sponge.state());

//         assert_eq!(trace_row9[11], to_element(2));
//         assert_eq!(trace_row10[11], to_element(1));

//         assert_eq!(trace_row10[12], to_element(2));
//     }

//     #[test]
//     fn test_stack_underflow() {
//         let source = "read2\nsadd";
//         let program = Program::compile(source).unwrap();
//         let error = Processor::run(program, program_inputs()).unwrap_err();

//         assert_eq!(
//             format!("{error}"),
//             format!("{}", StackError::stack_underflow("sadd", 2))
//         );
//     }
// }

fn server_key() -> ServerKey {
    let plaintext_modulus: u32 = 8u32;
    let ciphertext_modulus: u32 = 128u32;
    let k: usize = 4;
    let std = 2.412_390_240_121_573e-5;
    let parameters = LweParameters::new(plaintext_modulus, ciphertext_modulus, k, std);

    ServerKey::new(parameters)
}

fn program_inputs() -> ProgramInputs {
    let server_key = server_key();

    let clear_x = 33u8;
    let clear_y = 7u8;

    let a = 3u8;
    let b = 12u8;
    let x = server_key.encrypt(clear_x);
    let y = server_key.encrypt(clear_y);

    ProgramInputs::new(&[a, b], &[x, y], &server_key)
}

fn empty_program_inputs() -> ProgramInputs {
    ProgramInputs::new(&[], &[], &server_key())
}

fn to_element(value: u8) -> BaseElement {
    BaseElement::from(value)
}

fn to_elements(arr: &[u8]) -> Vec<BaseElement> {
    arr.iter().map(|value| to_element(*value)).collect()
}

fn trace_state(clk: usize, trace: &[Vec<BaseElement>]) -> Vec<BaseElement> {
    let mut state = Vec::with_capacity(trace.len());
    for col in trace {
        state.push(col[clk]);
    }
    state
}
