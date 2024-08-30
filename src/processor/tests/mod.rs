use super::*;

#[cfg(test)]
mod stack;

#[test]
fn test_execute_program() {
    let inputs = vec![3];
    let source = "push.1 push.2 add read mul";
    let program = Program::load(source).unwrap();
    let processor = Processor::run(program, ProgramInputs::new(&inputs)).unwrap();

    assert_eq!(processor.get_output(), 9);
}