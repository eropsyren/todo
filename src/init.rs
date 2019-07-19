use crate::constants::TODO_FILE_NAME;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use json;

pub fn init() {
    let file = File::create(TODO_FILE_NAME);

    match file {
        Ok(mut file) => match file.write_all(json::object!{}.dump().as_bytes()) {
            Ok(_) => (),
            Err(err) => eprintln!("error writing to .todo file: {}", err.description()), 
        },
        Err(err) => eprintln!("error generating .todo file: {}", err.description()),
    }
}
