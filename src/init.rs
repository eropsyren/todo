use crate::constants::TODO_FILE_NAME;
use json;
use std::fs::File;

pub fn init() {
    let file = File::create(TODO_FILE_NAME);

    match file {
        Ok(mut file) => match json::array![].write(&mut file) {
            Ok(_) => (),
            Err(err) => eprintln!("error writing to .todo file: {}", err),
        },
        Err(err) => eprintln!("error generating .todo file: {}", err),
    }
}
