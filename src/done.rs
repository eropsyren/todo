use crate::constants::{DONE, STATUS, TODO_FILE_NAME};
use json::JsonValue;
use std::fs::File;

pub fn done(id: &str) {
    let tasks = get_json_from_file_or_return!(TODO_FILE_NAME);
    let mut tasks = validate_json_or_return!(tasks, TODO_FILE_NAME);

    match &mut tasks[id] {
        JsonValue::Null => {
            print_error!("error: no task with {} id", id);
            return;
        }
        task => task[STATUS] = JsonValue::String(String::from(DONE)),
    }

    match File::create(TODO_FILE_NAME) {
        Ok(mut file) => match tasks.write(&mut file) {
            Ok(_) => (),
            Err(err) => print_error!("error writing to {}: {}", TODO_FILE_NAME, err),
        },
        Err(err) => print_error!("error rewriting {}: {}", TODO_FILE_NAME, err),
    }
}
