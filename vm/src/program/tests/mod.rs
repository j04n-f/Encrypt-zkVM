use std::io::Write;
use tempfile::NamedTempFile;

use super::Operation;
use super::Program;
use super::ProgramError;

#[cfg(test)]
mod parsers;

#[test]
fn test_read_program() {
    let mut tmpfile = NamedTempFile::new().unwrap();

    writeln!(tmpfile, "push.1").unwrap();
    writeln!(tmpfile, "push.2").unwrap();
    writeln!(tmpfile, "add").unwrap();
    writeln!(tmpfile, "read").unwrap();
    writeln!(tmpfile, "mul").unwrap();

    let path = tmpfile.into_temp_path();

    let program = Program::load(&path).unwrap();

    assert_eq!(
        format!("{program}"),
        String::from(
            "push(1) noop noop noop noop noop noop noop \
             push(2) add read mul noop noop noop"
        )
    );

    path.close().unwrap();
}

#[test]
fn test_invalid_op() {
    let source = "push.1 push.2 ad";
    let error = Program::compile(source).unwrap_err();

    assert_eq!(format!("{error}"), format!("{}", ProgramError::invalid_op(&["ad"], 3)));
}

#[test]
fn test_compile_program() {
    let source = "push.1 push.2 add read mul";
    let program = Program::compile(source).unwrap();

    assert_eq!(
        format!("{program}"),
        String::from(
            "push(1) noop noop noop noop noop noop noop \
             push(2) add read mul noop noop noop"
        )
    );
}

#[test]
fn test_program_padding() {
    let source = "push.1 push.2 add read read read mul add add";
    let program = Program::compile(source).unwrap();
    let code = program.get_code();

    assert_eq!(code.len() as u8 % 16, 0);
    assert_eq!(code[8], Operation::push(2));
    assert_eq!(code[14], Operation::noop());
    assert_eq!(code[15], Operation::noop());
}

#[test]
fn test_empty_program() {
    let source = "";
    let error = Program::compile(source).unwrap_err();

    assert_eq!(format!("{error}"), format!("{}", ProgramError::empty_program()));
}
