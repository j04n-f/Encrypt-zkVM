use super::*;

#[test]
fn test_decode_operation() {
    let mut decoder = Decoder::new(8);

    decoder.decode_op(&Operation::push(8));

    let decoder_trace = decoder.into_trace(8);

    assert_eq!(trace_state(0, &decoder_trace), to_elements(&[0, 0, 0, 0, 1]));
}

#[test]
fn test_fill_trace_with_noop() {
    let mut decoder = Decoder::new(8);

    decoder.decode_op(&Operation::push(8));

    let decoder_trace = decoder.into_trace(16);

    for i in 1..16 {
        assert_eq!(trace_state(i, &decoder_trace), to_elements(&[0, 0, 0, 0, 0]));
    }
}
