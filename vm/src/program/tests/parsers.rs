use super::*;

#[cfg(test)]
mod add {
    use super::*;

    #[test]
    fn test_parse() {
        let source = "add";
        let program = Program::compile(source).unwrap();
        let code = program.code();

        assert_eq!(code[0], Operation::add());
    }

    #[test]
    fn test_extra_param_error() {
        let source = "add.1";
        let error = Program::compile(source).unwrap_err();

        assert_eq!(
            format!("{error}"),
            format!("{}", ProgramError::extra_param(&["add"], 1))
        );
    }
}

#[cfg(test)]
mod mul {
    use super::*;

    #[test]
    fn test_parse() {
        let source = "mul";
        let program = Program::compile(source).unwrap();
        let code = program.code();

        assert_eq!(code[0], Operation::mul());
    }

    #[test]
    fn test_extra_param_error() {
        let source = "mul.1";
        let error = Program::compile(source).unwrap_err();

        assert_eq!(
            format!("{error}"),
            format!("{}", ProgramError::extra_param(&["mul"], 1))
        );
    }
}

#[cfg(test)]
mod smul {
    use super::*;

    #[test]
    fn test_parse() {
        let source = "smul";
        let program = Program::compile(source).unwrap();
        let code = program.code();

        assert_eq!(code[0], Operation::smul());
    }

    #[test]
    fn test_extra_param_error() {
        let source = "smul.1";
        let error = Program::compile(source).unwrap_err();

        assert_eq!(
            format!("{error}"),
            format!("{}", ProgramError::extra_param(&["smul"], 1))
        );
    }
}

#[cfg(test)]
mod sadd {
    use super::*;

    #[test]
    fn test_parse() {
        let source = "sadd";
        let program = Program::compile(source).unwrap();
        let code = program.code();

        assert_eq!(code[0], Operation::sadd());
    }

    #[test]
    fn test_extra_param_error() {
        let source = "sadd.1";
        let error = Program::compile(source).unwrap_err();

        assert_eq!(
            format!("{error}"),
            format!("{}", ProgramError::extra_param(&["sadd"], 1))
        );
    }
}

#[cfg(test)]
mod add2 {
    use super::*;

    #[test]
    fn test_parse() {
        let source = "add2";
        let program = Program::compile(source).unwrap();
        let code = program.code();

        assert_eq!(code[0], Operation::add2());
    }

    #[test]
    fn test_extra_param_error() {
        let source = "add2.1";
        let error = Program::compile(source).unwrap_err();

        assert_eq!(
            format!("{error}"),
            format!("{}", ProgramError::extra_param(&["add2"], 1))
        );
    }
}

#[cfg(test)]
mod read {
    use super::*;

    #[test]
    fn test_parse() {
        let source = "read";
        let program = Program::compile(source).unwrap();
        let code = program.code();

        assert_eq!(code[0], Operation::read());
    }

    #[test]
    fn test_extra_param_error() {
        let source = "read.1";
        let error = Program::compile(source).unwrap_err();

        assert_eq!(
            format!("{error}"),
            format!("{}", ProgramError::extra_param(&["read"], 1))
        );
    }
}

#[cfg(test)]
mod read2 {
    use super::*;

    #[test]
    fn test_parse() {
        let source = "read2";
        let program = Program::compile(source).unwrap();
        let code = program.code();

        assert_eq!(code[0], Operation::read2());
    }

    #[test]
    fn test_extra_param_error() {
        let source = "read2.1";
        let error = Program::compile(source).unwrap_err();

        assert_eq!(
            format!("{error}"),
            format!("{}", ProgramError::extra_param(&["read2"], 1))
        );
    }
}

#[cfg(test)]
mod push {
    use super::*;

    #[test]
    fn test_parse() {
        let source = "push.1";
        let program = Program::compile(source).unwrap();
        let code = program.code();

        assert_eq!(code[0], Operation::push(1));
    }

    #[test]
    fn test_extra_param_error() {
        let source = "push.1.1";
        let error = Program::compile(source).unwrap_err();

        assert_eq!(
            format!("{error}"),
            format!("{}", ProgramError::extra_param(&["push"], 1))
        );
    }

    #[test]
    fn test_missing_param_error() {
        let source = "push";
        let error = Program::compile(source).unwrap_err();

        assert_eq!(
            format!("{error}"),
            format!("{}", ProgramError::missing_param(&["push"], 1))
        );
    }
}
