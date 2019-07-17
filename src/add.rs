use crate::constants::TODO_FILE_NAME;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::Write;

pub fn add(task: &str) {
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
