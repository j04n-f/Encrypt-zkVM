use super::*;

#[test]
fn test_add() {
    let source = "push.1 push.2 add";
    let program = Program::load(source).unwrap();
    let processor = Processor::run(program, default_program_inputs()).unwrap();

    assert_eq!(vec![0, 0, 0, 0, 0], processor.stack.get_stack_state(0));
    assert_eq!(vec![1, 0, 0, 0, 0], processor.stack.get_stack_state(1));
    assert_eq!(vec![2, 1, 0, 0, 0], processor.stack.get_stack_state(2));
    assert_eq!(vec![3, 0, 0, 0, 0], processor.stack.get_stack_state(3));

    assert_eq!(processor.get_output()[0], 3);
}

#[test]
fn test_mul() {
    let source = "push.1 push.2 mul";
    let program = Program::load(source).unwrap();
    let processor = Processor::run(program, default_program_inputs()).unwrap();

    assert_eq!(vec![0, 0, 0, 0, 0], processor.stack.get_stack_state(0));
    assert_eq!(vec![1, 0, 0, 0, 0], processor.stack.get_stack_state(1));
    assert_eq!(vec![2, 1, 0, 0, 0], processor.stack.get_stack_state(2));
    assert_eq!(vec![2, 0, 0, 0, 0], processor.stack.get_stack_state(3));

    assert_eq!(processor.get_output()[0], 2);
}

#[test]
fn test_read() {
    let source = "read";
    let program = Program::load(source).unwrap();
    let processor = Processor::run(program, default_program_inputs()).unwrap();

    assert_eq!(vec![0, 0, 0, 0, 0], processor.stack.get_stack_state(0));
    assert_eq!(vec![3, 0, 0, 0, 0], processor.stack.get_stack_state(1));
}

#[test]
fn test_read2() {
    let source = "read2";
    let inputs = default_program_inputs();
    let value = &inputs.get_secret()[0];
    let program = Program::load(source).unwrap();
    let processor = Processor::run(program, inputs).unwrap();

    assert_eq!(vec![0, 0, 0, 0, 0], processor.stack.get_stack_state(0));
    assert_eq!(value.ciphertext(), processor.stack.get_stack_state(1));
}

#[test]
fn test_push() {
    let source = "push.4";
    let program = Program::load(source).unwrap();
    let processor = Processor::run(program, default_program_inputs()).unwrap();

    assert_eq!(vec![0, 0, 0, 0, 0], processor.stack.get_stack_state(0));
    assert_eq!(vec![4, 0, 0, 0, 0], processor.stack.get_stack_state(1));
}

#[test]
fn test_sadd() {
    let source = "read2 read sadd";
    let inputs = default_program_inputs();
    let value = &inputs.get_secret()[0];
    let scalar = &inputs.get_public()[0];
    let server_key = inputs.get_server_key();
    let program = Program::load(source).unwrap();
    let processor = Processor::run(program, inputs).unwrap();

    let result = server_key.scalar_add(scalar, value);

    let mut register1 = value.ciphertext();
    register1.push(0);

    let mut register2 = vec![*scalar as u128];
    register2.extend(value.ciphertext());

    let mut register3 = result.ciphertext();
    register3.push(0);

    assert_eq!(vec![0, 0, 0, 0, 0, 0], processor.stack.get_stack_state(0));
    assert_eq!(register1, processor.stack.get_stack_state(1));
    assert_eq!(register2, processor.stack.get_stack_state(2));
    assert_eq!(register3, processor.stack.get_stack_state(3));
}

#[test]
fn test_smul() {
    let source = "read2 read smul";
    let inputs = default_program_inputs();
    let value = &inputs.get_secret()[0];
    let scalar = &inputs.get_public()[0];
    let server_key = inputs.get_server_key();
    let program = Program::load(source).unwrap();
    let processor = Processor::run(program, inputs).unwrap();

    let result = server_key.scalar_mul(scalar, value);

    let mut register1 = value.ciphertext();
    register1.push(0);

    let mut register2 = vec![*scalar as u128];
    register2.extend(value.ciphertext());

    let mut register3 = result.ciphertext();
    register3.push(0);

    assert_eq!(vec![0, 0, 0, 0, 0, 0], processor.stack.get_stack_state(0));
    assert_eq!(register1, processor.stack.get_stack_state(1));
    assert_eq!(register2, processor.stack.get_stack_state(2));
    assert_eq!(register3, processor.stack.get_stack_state(3));
}

#[test]
fn test_add_stack_underflow() {
    let source = "push.1 add";
    let program = Program::load(source).unwrap();
    let error = Processor::run(program, default_program_inputs()).unwrap_err();

    assert_eq!(
        format!("{error}"),
        format!("{}", StackError::stack_underflow(OpCode::Add, 2))
    );
}

#[test]
fn test_sadd_stack_underflow() {
    let source = "read2 sadd";
    let program = Program::load(source).unwrap();
    let error = Processor::run(program, default_program_inputs()).unwrap_err();

    assert_eq!(
        format!("{error}"),
        format!("{}", StackError::stack_underflow(OpCode::SAdd, 2))
    );
}

#[test]
fn test_mul_stack_underflow() {
    let source = "push.1 mul";
    let program = Program::load(source).unwrap();
    let error = Processor::run(program, default_program_inputs()).unwrap_err();

    assert_eq!(
        format!("{error}"),
        format!("{}", StackError::stack_underflow(OpCode::Mul, 2))
    );
}

#[test]
fn test_smul_stack_underflow() {
    let source = "read2 smul";
    let program = Program::load(source).unwrap();
    let error = Processor::run(program, default_program_inputs()).unwrap_err();

    assert_eq!(
        format!("{error}"),
        format!("{}", StackError::stack_underflow(OpCode::SAdd, 2))
    );
}

#[test]
fn test_read_empty_inputs() {
    let source = "read read read";
    let program = Program::load(source).unwrap();
    let error = Processor::run(program, default_program_inputs()).unwrap_err();

    assert_eq!(
        format!("{error}"),
        format!("{}", StackError::empty_inputs(3))
    );
}

#[test]
fn test_read2_empty_inputs() {
    let source = "read2 read2";
    let program = Program::load(source).unwrap();
    let error = Processor::run(program, default_program_inputs()).unwrap_err();

    assert_eq!(
        format!("{error}"),
        format!("{}", StackError::empty_inputs(2))
    );
}
