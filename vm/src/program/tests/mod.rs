use super::AssemblyError;
use super::Program;

#[cfg(test)]
mod parsers;

#[test]
fn test_valid_program() {
    let source = "push.1 push.2 add read mul";
    let program = Program::load(source).unwrap();

    assert_eq!(
        format!("{program}"),
        String::from("push(1) push(2) add read mul")
    );
}

#[test]
fn test_empty_program() {
    let source = "";
    let error = Program::load(source).unwrap_err();

    assert_eq!(
        format!("{error}"),
        format!("{}", AssemblyError::empty_program())
    );
}
