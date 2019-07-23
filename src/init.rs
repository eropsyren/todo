use crate::constants::TODO_FILE_NAME;
use json;
use std::fs::File;

pub fn init() {
    let file = File::create(TODO_FILE_NAME);

    match file {
        Ok(mut file) => match json::object![].write(&mut file) {
            Ok(_) => (),
            Err(err) => print_error!("error writing to {} file: {}", TODO_FILE_NAME, err),
        },
        Err(err) => print_error!("error generating {} file: {}", TODO_FILE_NAME, err),
    }
}
