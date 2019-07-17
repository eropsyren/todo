use std::error::Error;
use std::fs::File;
use std::path::Path;

const TODO_FILE_NAME: &'static str = ".todo";

pub fn init() {
    let already_exists = Path::new(TODO_FILE_NAME).exists();

    if already_exists {
        eprintln!("error: there is a .todo file already");
    } else {
        let f = File::create(".todo");

        match f {
            Ok(_) => (),
            Err(error) => eprintln!("error generating .todo file: {}", error.description()),
        }
    }
}
