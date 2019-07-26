use crate::constants::{DISCARDED, DONE, STATUS, TODO_FILE_NAME, UNDONE};
use json::JsonValue;

pub fn clear() {
    let tasks = get_json_from_file_or_exit!(TODO_FILE_NAME);
    let mut tasks = is_object_or_exit!(tasks, TODO_FILE_NAME);
    let to_remove: Vec<String> = tasks
        .entries()
        .filter(|(_, task)| {
            let status = get_prop_or_exit!(task, STATUS, JsonValue::Short, JsonValue::String);
            let status = status.to_string();

            match status.as_str() {
                DONE => true,
                UNDONE | DISCARDED => false,
                invalid => {
                    print_error!("error: '{}' is an invalid status", invalid);
                    false
                }
            }
        })
        .map(|(id, _)| String::from(id))
        .collect();

    for id in to_remove {
        tasks.remove(&id);
    }

    write_json_to_file_or_err!(tasks, TODO_FILE_NAME);
}
