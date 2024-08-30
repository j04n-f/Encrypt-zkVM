use super::*;

#[test]
fn test_invalid_op() {
    let source = "push.1 push.2 ad";
    let error = Program::load(source).unwrap_err();

    assert_eq!(
        format!("{error}"),
        format!("{}", AssemblyError::invalid_op(&["ad"], 3))
    );
}

#[test]
fn test_extra_add_param() {
    let source = "push.1 push.2 add.1";
    let error = Program::load(source).unwrap_err();

    assert_eq!(
        format!("{error}"),
        format!("{}", AssemblyError::extra_param(&["add"], 3))
    );
}

#[test]
fn test_extra_mul_param() {
    let source = "push.1 push.2 mul.1";
    let error = Program::load(source).unwrap_err();

    assert_eq!(
        format!("{error}"),
        format!("{}", AssemblyError::extra_param(&["mul"], 3))
    );
}

#[test]
fn test_extra_read_param() {
    let source = "push.1 push.2 read.1";
    let error = Program::load(source).unwrap_err();

    assert_eq!(
        format!("{error}"),
        format!("{}", AssemblyError::extra_param(&["read"], 3))
    );
}

#[test]
fn test_extra_push_param() {
    let source = "push.1 push.2.2 read.1";
    let error = Program::load(source).unwrap_err();

    assert_eq!(
        format!("{error}"),
        format!("{}", AssemblyError::extra_param(&["push"], 2))
    );
}

#[test]
fn test_missing_push_param() {
    let source = "push.1 push read.1";
    let error = Program::load(source).unwrap_err();

    assert_eq!(
        format!("{error}"),
        format!("{}", AssemblyError::missing_param(&["push"], 2))
    );
}