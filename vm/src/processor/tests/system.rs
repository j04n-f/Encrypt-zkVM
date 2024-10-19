use super::*;

#[test]
fn test_advance_clock() {
    let mut system = System::new(8);

    for _ in 0..8 {
        system.advance_step();
    }

    let clk_trace = system.into_trace(8)[0].to_vec();

    assert_eq!(clk_trace, to_elements(&[0, 1, 2, 3, 4, 5, 6, 7]));
}

#[test]
fn test_incremental_trace_fill() {
    let mut system = System::new(8);

    for _ in 0..16 {
        system.advance_step();
    }

    let clk_trace = system.into_trace(16);

    for i in 0..16 {
        assert_eq!(trace_state(i, &clk_trace), to_elements(&[i as u8]));
    }
}
