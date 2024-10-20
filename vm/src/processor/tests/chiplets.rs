use crypto::{rescue::CYCLE_LENGTH, Rescue128};
use errors::ChipletsError;

use super::*;

#[test]
fn test_hash_operation() {
    let mut chiplets = Chiplets::new(8);

    let mut sponge = Rescue128::new();

    for _ in 0..14 {
        chiplets.hash_op(&Operation::push(2)).unwrap();
    }

    for _ in 0..2 {
        chiplets.hash_op(&Operation::noop()).unwrap();
    }

    let chiplets_trace = chiplets.into_trace(32).unwrap();

    for i in 0..14 {
        let trace = trace_state(i, &chiplets_trace);
        assert_eq!(trace[0], ONE);
        assert_eq!(trace[1..5], sponge.state());
        let op = Operation::push(2);
        sponge.update(op.code(), op.value());
    }

    for i in 0..2 {
        let trace = trace_state(14 + i, &chiplets_trace);
        assert_eq!(trace[0], ONE);
        assert_eq!(trace[1..5], sponge.state());
        let op = Operation::noop();
        sponge.update(op.code(), op.value());
    }

    let trace_row31 = trace_state(31, &chiplets_trace);

    assert_eq!(trace_row31[0], ZERO);
    assert_eq!(trace_row31[1..5], sponge.state());
}

#[test]
fn test_fill_trace_with_last_value() {
    let mut chiplets = Chiplets::new(8);

    for _ in 0..14 {
        chiplets.hash_op(&Operation::push(2)).unwrap();
    }

    for _ in 0..2 {
        chiplets.hash_op(&Operation::noop()).unwrap();
    }

    let chiplets_trace = chiplets.into_trace(32).unwrap();

    for i in 16..32 {
        let trace_row = trace_state(i, &chiplets_trace);

        assert_eq!(trace_row[0], ZERO);
        assert_eq!(trace_row[1..5], trace_state(i - 1, &chiplets_trace)[1..5]);
    }
}

#[test]
fn test_invalid_operation_error() {
    let mut chiplets = Chiplets::new(8);

    for _ in 0..14 {
        chiplets.hash_op(&Operation::push(2)).unwrap();
    }

    let op = Operation::push(2);

    let error = chiplets.hash_op(&op).unwrap_err();

    assert_eq!(
        format!("{error}"),
        format!("{}", ChipletsError::invalid_operation(&op, 15))
    );
}

#[test]
fn test_invalid_trace_length_error() {
    let mut chiplets = Chiplets::new(8);

    for _ in 0..8 {
        chiplets.hash_op(&Operation::push(2)).unwrap();
    }

    let error = chiplets.into_trace(32).unwrap_err();

    assert_eq!(
        format!("{error}"),
        format!("{}", ChipletsError::invalid_trace_length(CYCLE_LENGTH, 8, 8))
    );
}
