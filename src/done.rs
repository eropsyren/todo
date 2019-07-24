use crate::constants::{DISCARDED, DONE, STATUS, SUBTASKS, TODO_FILE_NAME, UNDONE};
use json::JsonValue;
use std::process;

pub fn done(id: &str) {
    let tasks = get_json_from_file_or_exit!(TODO_FILE_NAME);
    let mut tasks = is_object_or_exit!(tasks, TODO_FILE_NAME);

    if tasks.has_key(id) {
        for (_, task) in tasks[id][SUBTASKS].entries() {
            let status =
                get_prop_or_exit!(task, STATUS, JsonValue::Short, JsonValue::String).to_string();

            match status.as_str() {
                DONE | DISCARDED => (),
                UNDONE => {
                    print_error!("error: one of the subtasks is {}", UNDONE);
                    process::exit(1);
                }
                val => {
                    print_error!("error: invalid status '{}'", val);
                    process::exit(1);
                }
            }
        }

        tasks[id][STATUS] = JsonValue::String(String::from(DONE));
    } else {
        for (_, task) in tasks.entries_mut() {
            if task[SUBTASKS].has_key(id) {
                task[SUBTASKS][id][STATUS] = JsonValue::String(String::from(DONE));

                break;
            }
        }
    }

    write_json_to_file_or_err!(tasks, TODO_FILE_NAME);
}
