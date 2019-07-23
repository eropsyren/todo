use crate::constants::{DONE, STATUS, TODO_FILE_NAME};
use json::JsonValue;

pub fn done(id: &str) {
    let tasks = get_json_from_file_or_exit!(TODO_FILE_NAME);
    let mut tasks = is_object_or_exit!(tasks, TODO_FILE_NAME);

    match &mut tasks[id] {
        JsonValue::Null => {
            print_error!("error: no task with {} id", id);

            return;
        }
        task => task[STATUS] = JsonValue::String(String::from(DONE)),
    }

    write_json_to_file_or_err!(tasks, TODO_FILE_NAME);
}
