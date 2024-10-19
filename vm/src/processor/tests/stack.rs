use super::*;

use errors::StackError;

#[test]
fn test_fill_trace_with_noop() {
    let mut stack = Stack::new(&program_inputs(), 8);

    for _ in 0..4 {
        stack.execute_op(&Operation::push(2)).unwrap();
        stack.execute_op(&Operation::push(2)).unwrap();
        stack.execute_op(&Operation::add()).unwrap();
    }

    let stack_trace = stack.into_trace(16);

    for i in 12..15 {
        assert_eq!(trace_state(i, &stack_trace), trace_state(i + 1, &stack_trace));
    }
}

mod mul {

    use super::*;

    #[test]
    fn test_operation_execution() {
        let mut stack = Stack::new(&program_inputs(), 8);

        stack.execute_op(&Operation::push(2)).unwrap();
        stack.execute_op(&Operation::push(2)).unwrap();
        stack.execute_op(&Operation::mul()).unwrap();

        let stack_trace = stack.into_trace(8);

        let trace_row2 = trace_state(2, &stack_trace);
        let trace_row3 = trace_state(3, &stack_trace);

        assert_eq!(trace_row2[0], to_element(2));
        assert_eq!(trace_row3[0], to_element(1));

        assert_eq!(trace_row3[1], to_element(4));
    }

    #[test]
    fn test_stack_underflow_error() {
        let mut stack = Stack::new(&program_inputs(), 8);

        stack.execute_op(&Operation::push(2)).unwrap();

        let op = Operation::mul();

        let error = stack.execute_op(&op).unwrap_err();

        assert_eq!(format!("{error}"), format!("{}", StackError::stack_underflow(&op, 2)));
    }
}

mod add {

    use super::*;

    #[test]
    fn test_operation_execution() {
        let mut stack = Stack::new(&program_inputs(), 8);

        stack.execute_op(&Operation::push(2)).unwrap();
        stack.execute_op(&Operation::push(2)).unwrap();
        stack.execute_op(&Operation::add()).unwrap();

        let stack_trace = stack.into_trace(8);

        let trace_row2 = trace_state(2, &stack_trace);
        let trace_row3 = trace_state(3, &stack_trace);

        assert_eq!(trace_row2[0], to_element(2));
        assert_eq!(trace_row3[0], to_element(1));

        assert_eq!(trace_row3[1], to_element(4));
    }

    #[test]
    fn test_stack_underflow_error() {
        let mut stack = Stack::new(&program_inputs(), 8);

        stack.execute_op(&Operation::push(2)).unwrap();

        let op = Operation::add();

        let error = stack.execute_op(&op).unwrap_err();

        assert_eq!(format!("{error}"), format!("{}", StackError::stack_underflow(&op, 2)));
    }
}

mod noop {

    use super::*;

    #[test]
    fn test_operation_execution() {
        let mut stack = Stack::new(&program_inputs(), 8);

        stack.execute_op(&Operation::noop()).unwrap();

        let stack_trace = stack.into_trace(8);

        let trace_row0 = trace_state(0, &stack_trace);
        let trace_row1 = trace_state(1, &stack_trace);

        assert_eq!(trace_row0, trace_row1);
    }
}

mod push {

    use super::*;

    #[test]
    fn test_operation_execution() {
        let mut stack = Stack::new(&program_inputs(), 8);

        stack.execute_op(&Operation::push(5)).unwrap();

        let stack_trace = stack.into_trace(8);

        let trace_row0 = trace_state(0, &stack_trace);
        let trace_row1 = trace_state(1, &stack_trace);

        assert_eq!(trace_row0[0], to_element(0));
        assert_eq!(trace_row1[0], to_element(1));

        assert_eq!(trace_row1[1], to_element(5));
    }
}

mod read {

    use super::*;

    #[test]
    fn test_operation_execution() {
        let mut stack = Stack::new(&program_inputs(), 8);

        stack.execute_op(&Operation::read()).unwrap();

        let stack_trace = stack.into_trace(8);

        let trace_row0 = trace_state(0, &stack_trace);
        let trace_row1 = trace_state(1, &stack_trace);

        assert_eq!(trace_row0[0], to_element(0));
        assert_eq!(trace_row1[0], to_element(1));

        assert_eq!(trace_row1[1], to_element(3));
    }

    #[test]
    fn test_empty_inputs_error() {
        let mut stack = Stack::new(&empty_program_inputs(), 8);

        let op = Operation::read();

        let error = stack.execute_op(&op).unwrap_err();

        assert_eq!(format!("{error}"), format!("{}", StackError::empty_inputs(&op, 1)));
    }
}

mod read2 {

    use super::*;

    #[test]
    fn test_operation_execution() {
        let inputs = program_inputs();
        let sct_inputs = inputs.get_secret().to_vec();

        let mut stack = Stack::new(&inputs, 8);

        stack.execute_op(&Operation::read2()).unwrap();

        let stack_trace = stack.into_trace(8);

        let trace_row0 = trace_state(0, &stack_trace);
        let trace_row1 = trace_state(1, &stack_trace);

        assert_eq!(trace_row0[0], to_element(0));
        assert_eq!(trace_row1[0], to_element(5));

        let input_ct = sct_inputs[0].ciphertext().to_vec();

        assert_eq!(trace_row1[1..6], input_ct);
    }

    #[test]
    fn test_empty_inputs_error() {
        let mut stack = Stack::new(&empty_program_inputs(), 8);

        let op = Operation::read2();

        let error = stack.execute_op(&op).unwrap_err();

        assert_eq!(format!("{error}"), format!("{}", StackError::empty_inputs(&op, 1)));
    }
}

mod sadd {

    use super::*;

    #[test]
    fn test_operation_execution() {
        let inputs = program_inputs();
        let pub_inputs = inputs.get_public().to_vec();
        let sct_inputs = inputs.get_secret().to_vec();

        let mut stack = Stack::new(&inputs, 8);

        stack.execute_op(&Operation::read2()).unwrap();
        stack.execute_op(&Operation::read()).unwrap();
        stack.execute_op(&Operation::sadd()).unwrap();

        let stack_trace = stack.into_trace(8);

        let trace_row2 = trace_state(2, &stack_trace);
        let trace_row3 = trace_state(3, &stack_trace);

        assert_eq!(trace_row2[0], to_element(6));
        assert_eq!(trace_row3[0], to_element(5));

        let scalar = BaseElement::from(pub_inputs[0]);

        let result = inputs.get_server_key().scalar_add(&scalar, &sct_inputs[0]);
        let result_ct = result.ciphertext().to_vec();

        assert_eq!(trace_row3[1..6], result_ct);
    }

    #[test]
    fn test_stack_underflow_error() {
        let mut stack = Stack::new(&program_inputs(), 8);

        stack.execute_op(&Operation::read2()).unwrap();

        let op = Operation::sadd();

        let error = stack.execute_op(&op).unwrap_err();

        assert_eq!(format!("{error}"), format!("{}", StackError::stack_underflow(&op, 2)));
    }
}

mod smul {

    use super::*;

    #[test]
    fn test_operation_execution() {
        let inputs = program_inputs();
        let pub_inputs = inputs.get_public().to_vec();
        let sct_inputs = inputs.get_secret().to_vec();

        let mut stack = Stack::new(&inputs, 8);

        stack.execute_op(&Operation::read2()).unwrap();
        stack.execute_op(&Operation::read()).unwrap();
        stack.execute_op(&Operation::smul()).unwrap();

        let stack_trace = stack.into_trace(8);

        let trace_row2 = trace_state(2, &stack_trace);
        let trace_row3 = trace_state(3, &stack_trace);

        assert_eq!(trace_row2[0], to_element(6));
        assert_eq!(trace_row3[0], to_element(5));

        let scalar = BaseElement::from(pub_inputs[0]);

        let result = inputs.get_server_key().scalar_mul(&scalar, &sct_inputs[0]);
        let result_ct = result.ciphertext().to_vec();

        assert_eq!(trace_row3[1..6], result_ct);
    }

    #[test]
    fn test_stack_underflow_error() {
        let mut stack = Stack::new(&program_inputs(), 8);

        stack.execute_op(&Operation::read2()).unwrap();

        let op = Operation::smul();

        let error = stack.execute_op(&op).unwrap_err();

        assert_eq!(format!("{error}"), format!("{}", StackError::stack_underflow(&op, 2)));
    }
}

mod add2 {

    use super::*;

    #[test]
    fn test_operation_execution() {
        let inputs = program_inputs();
        let sct_inputs = inputs.get_secret().to_vec();

        let mut stack = Stack::new(&inputs, 8);

        stack.execute_op(&Operation::read2()).unwrap();
        stack.execute_op(&Operation::read2()).unwrap();
        stack.execute_op(&Operation::add2()).unwrap();

        let stack_trace = stack.into_trace(8);

        let trace_row2 = trace_state(2, &stack_trace);
        let trace_row3 = trace_state(3, &stack_trace);

        assert_eq!(trace_row2[0], to_element(10));
        assert_eq!(trace_row3[0], to_element(5));

        let result = inputs.get_server_key().add(&sct_inputs[0], &sct_inputs[1]);
        let result_ct = result.ciphertext().to_vec();

        assert_eq!(trace_row3[1..6], result_ct);
    }

    #[test]
    fn test_stack_underflow_error() {
        let mut stack = Stack::new(&program_inputs(), 8);

        stack.execute_op(&Operation::read2()).unwrap();

        let op = Operation::add2();

        let error = stack.execute_op(&op).unwrap_err();

        assert_eq!(format!("{error}"), format!("{}", StackError::stack_underflow(&op, 2)));
    }
}
