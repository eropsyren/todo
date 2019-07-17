use crate::constants::TODO_FILE_NAME;
use std::error::Error;
use std::fs::File;

pub fn init() {
    let f = File::create(TODO_FILE_NAME);

    match f {
        Ok(_) => (),
        Err(error) => eprintln!("error generating .todo file: {}", error.description()),
    }
}
