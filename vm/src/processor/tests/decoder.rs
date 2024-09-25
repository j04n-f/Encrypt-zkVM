use super::*;

// TODO: Into Trace Test

#[test]
fn test_decode_op() {
    let source = "push.1 push.2 add";
    let program = Program::compile(source).unwrap();
    let processor = Processor::run(program, default_program_inputs()).unwrap();

    assert_eq!(vec![0, 1, 1], processor.decoder.decoder_state(0));
    assert_eq!(vec![0, 1, 1], processor.decoder.decoder_state(1));
    assert_eq!(vec![1, 0, 1], processor.decoder.decoder_state(2));
    assert_eq!(vec![0, 0, 0], processor.decoder.decoder_state(3));
}
