use crate::constants::{DISCARDED, DONE, STATUS, TODO_FILE_NAME, UNDONE};
use json::JsonValue;
use std::process;

pub fn remove(id: &str) {
    let tasks = get_json_from_file_or_exit!(TODO_FILE_NAME);
    let mut tasks = is_object_or_exit!(tasks, TODO_FILE_NAME);
    let task = get_prop_or_exit!(tasks, id, JsonValue::Object);
    let status = get_prop_or_exit!(task, STATUS, JsonValue::String, JsonValue::Short).to_string();

    match status.as_str() {
        DONE | DISCARDED => {
            tasks.remove(id);
        }
        s @ UNDONE => {
            print_error!("error: cannot remove a {} task", s);
            process::exit(1);
        }
        s => {
            print_error!("error: task has unvalid status {}", s);
            process::exit(1);
        }
    }

    write_json_to_file_or_err!(tasks, TODO_FILE_NAME);
}
