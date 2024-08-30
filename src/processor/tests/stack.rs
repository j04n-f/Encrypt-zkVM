use super::*;

#[test]
fn test_add() {
    let source = "push.1 push.2 add";
    let program = Program::load(source).unwrap();
    let processor = Processor::run(program, ProgramInputs::none()).unwrap();

    assert_eq!(vec![0, 0, 0, 0, 0], processor.stack.get_stack_state(0));
    assert_eq!(vec![1, 0, 0, 0, 0], processor.stack.get_stack_state(1));
    assert_eq!(vec![2, 1, 0, 0, 0], processor.stack.get_stack_state(2));
    assert_eq!(vec![3, 0, 0, 0, 0], processor.stack.get_stack_state(3));

    assert_eq!(processor.get_output(), 3);
}

#[test]
fn test_mul() {
    let source = "push.1 push.2 mul";
    let program = Program::load(source).unwrap();
    let processor = Processor::run(program, ProgramInputs::none()).unwrap();

    assert_eq!(vec![0, 0, 0, 0, 0], processor.stack.get_stack_state(0));
    assert_eq!(vec![1, 0, 0, 0, 0], processor.stack.get_stack_state(1));
    assert_eq!(vec![2, 1, 0, 0, 0], processor.stack.get_stack_state(2));
    assert_eq!(vec![2, 0, 0, 0, 0], processor.stack.get_stack_state(3));

    assert_eq!(processor.get_output(), 2);
}

#[test]
fn test_read() {
  let inputs = vec![3];
  let source = "read";
  let program = Program::load(source).unwrap();
  let processor = Processor::run(program, ProgramInputs::new(&inputs)).unwrap();

  assert_eq!(vec![0, 0, 0, 0, 0], processor.stack.get_stack_state(0));
  assert_eq!(vec![3, 0, 0, 0, 0], processor.stack.get_stack_state(1));
}

#[test]
fn test_push() {
  let inputs = vec![3];
  let source = "push.4";
  let program = Program::load(source).unwrap();
  let processor = Processor::run(program, ProgramInputs::new(&inputs)).unwrap();

  assert_eq!(vec![0, 0, 0, 0, 0], processor.stack.get_stack_state(0));
  assert_eq!(vec![4, 0, 0, 0, 0], processor.stack.get_stack_state(1));
}

#[test]
fn test_add_stack_underflow() {
    let inputs = vec![3];
    let source = "push.1 add";
    let program = Program::load(source).unwrap();
    let error = Processor::run(program, ProgramInputs::new(&inputs)).unwrap_err();

    assert_eq!(
        format!("{error}"),
        format!("{}", StackError::stack_underflow(OpCode::Add, 2))
    );
}

#[test]
fn test_mul_stack_underflow() {
    let inputs = vec![3];
    let source = "push.1 mul";
    let program = Program::load(source).unwrap();
    let error = Processor::run(program, ProgramInputs::new(&inputs)).unwrap_err();

    assert_eq!(
        format!("{error}"),
        format!("{}", StackError::stack_underflow(OpCode::Mul, 2))
    );
}

#[test]
fn test_read_empty_inputs() {
    let source = "read";
    let program = Program::load(source).unwrap();
    let error = Processor::run(program, ProgramInputs::none()).unwrap_err();

    assert_eq!(
        format!("{error}"),
        format!("{}", StackError::empty_inputs(1))
    );
}
