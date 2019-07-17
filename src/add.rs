use crate::constants::TODO_FILE_NAME;
use std::error::Error;
use std::fs::OpenOptions;
use std::io;
use std::io::Write;

pub fn add() {
    print!("enter task: ");

    match io::stdout().flush() {
        Ok(_) => (),
        Err(err) => eprintln!("error flushing stdout: {}", err.description()),
    }

    let mut input = String::new();

    match io::stdin().read_line(&mut input) {
        Ok(_) => append_task(&input),
        Err(error) => {
            eprintln!("error reding from standard input: {}", error.description());
            return;
        }
    }
}

fn append_task(task: &str) {
    let file = OpenOptions::new()
        .read(true)
        .append(true)
        .open(TODO_FILE_NAME);

    match file {
        Ok(mut file) => match write!(&mut file, "{}", task) {
            Ok(_) => (),
            Err(err) => eprintln!(
                "error writing task to {}: {}",
                TODO_FILE_NAME,
                err.description()
            ),
        },
        Err(err) => eprintln!(
            "error reading file {}: {}",
            TODO_FILE_NAME,
            err.description()
        ),
    }
}
