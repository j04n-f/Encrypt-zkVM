// use super::*;

// TODO: Into Trace Test

// #[test]
// fn test_decode_op() {
//     let source = "push.1 push.2 add";
//     let program = Program::compile(source).unwrap();
//     let processor = Processor::run(program, default_program_inputs()).unwrap();

//     assert_eq!(vec![ZERO, ONE, ONE], processor.decoder.decoder_bits_state(0));
//     assert_eq!(vec![ZERO, ONE, ONE], processor.decoder.decoder_bits_state(1));
//     assert_eq!(vec![ONE, ZERO, ONE], processor.decoder.decoder_bits_state(2));
//     assert_eq!(vec![ZERO, ZERO, ZERO], processor.decoder.decoder_bits_state(3));
// }
