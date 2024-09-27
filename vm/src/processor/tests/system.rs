use super::*;

#[test]
fn test_clock() {
    let source = "push.1 push.2 mul";
    let program = Program::compile(source).unwrap();
    let processor = Processor::run(program, default_program_inputs()).unwrap();

    assert_eq!(to_element(0), processor.system.system_state(0));
    assert_eq!(to_element(1), processor.system.system_state(1));
    assert_eq!(to_element(2), processor.system.system_state(2));
    assert_eq!(to_element(3), processor.system.system_state(3));
}

#[test]
fn test_trace() {
    let source = "push.1 push.2 mul";
    let program = Program::compile(source).unwrap();
    let processor = Processor::run(program, default_program_inputs()).unwrap();

    let trace_length = processor.system.trace_length();

    assert_eq!(
        to_elements(vec![0, 1, 2, 3, 4, 5, 6, 7]),
        processor.system.into_trace(trace_length)[0]
    );

    let source = "push.1 push.2 mul push.2 push.3 mul push.2 mul";
    let program = Program::compile(source).unwrap();
    let processor = Processor::run(program, default_program_inputs()).unwrap();

    let trace_length = processor.system.trace_length();

    assert_eq!(
        to_elements(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]),
        processor.system.into_trace(trace_length)[0]
    );
}
