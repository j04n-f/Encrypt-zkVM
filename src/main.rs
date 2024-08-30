use std::fs;

mod program;
use program::{Program, ProgramInputs};

mod processor;
use processor::Processor;

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

    let processor = match Processor::run(program, ProgramInputs::new(&[])) {
        Ok(output) => output,
        Err(err) => return Err(format!("{err}")),
    };

    println!("Output is {}", processor.get_output());

    Ok(())
}
