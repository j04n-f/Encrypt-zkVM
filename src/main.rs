use std::fs;

mod program;
use program::Program;

fn main() -> Result<(), String> {
    let path = "prog.rm";

    let source = match fs::read_to_string(path) {
        Ok(source) => source,
        Err(err) => {
            return Err(format!(
                "could not open {}: {}",
                path,
                err.to_string().to_lowercase()
            ))
        }
    };

    let program = match Program::load(&source) {
        Ok(program) => program,
        Err(err) => return Err(format!("{err}")),
    };

    Ok(())
}
