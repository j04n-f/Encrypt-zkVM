use super::*;

#[test]
fn test_invalid_op() {
    let source = "push.1 push.2 ad";
    let error = Program::compile(source).unwrap_err();

    assert_eq!(format!("{error}"), format!("{}", ProgramError::invalid_op(&["ad"], 3)));
}

#[test]
fn test_extra_add_param() {
    let source = "push.1 push.2 add.1";
    let error = Program::compile(source).unwrap_err();

    assert_eq!(
        format!("{error}"),
        format!("{}", ProgramError::extra_param(&["add"], 3))
    );
}

#[test]
fn test_extra_sadd_param() {
    let source = "push.1 push.2 sadd.1";
    let error = Program::compile(source).unwrap_err();

    assert_eq!(
        format!("{error}"),
        format!("{}", ProgramError::extra_param(&["sadd"], 3))
    );
}

#[test]
fn test_extra_mul_param() {
    let source = "push.1 push.2 mul.1";
    let error = Program::compile(source).unwrap_err();

    assert_eq!(
        format!("{error}"),
        format!("{}", ProgramError::extra_param(&["mul"], 3))
    );
}

#[test]
fn test_extra_smul_param() {
    let source = "push.1 push.2 smul.1";
    let error = Program::compile(source).unwrap_err();

    assert_eq!(
        format!("{error}"),
        format!("{}", ProgramError::extra_param(&["smul"], 3))
    );
}

#[test]
fn test_extra_read_param() {
    let source = "push.1 push.2 read.1";
    let error = Program::compile(source).unwrap_err();

    assert_eq!(
        format!("{error}"),
        format!("{}", ProgramError::extra_param(&["read"], 3))
    );
}

#[test]
fn test_extra_read2_param() {
    let source = "push.1 push.2 read2.1";
    let error = Program::compile(source).unwrap_err();

    assert_eq!(
        format!("{error}"),
        format!("{}", ProgramError::extra_param(&["read2"], 3))
    );
}

#[test]
fn test_extra_push_param() {
    let source = "push.1 push.2.2 read.1";
    let error = Program::compile(source).unwrap_err();

    assert_eq!(
        format!("{error}"),
        format!("{}", ProgramError::extra_param(&["push"], 2))
    );
}

#[test]
fn test_missing_push_param() {
    let source = "push.1 push read.1";
    let error = Program::compile(source).unwrap_err();

    assert_eq!(
        format!("{error}"),
        format!("{}", ProgramError::missing_param(&["push"], 2))
    );
}
