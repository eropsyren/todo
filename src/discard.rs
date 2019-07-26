use crate::constants::{DISCARDED, DONE, STATUS, TODO_FILE_NAME};
use json::JsonValue;

pub fn discard(id: &str) {
    let tasks = get_json_from_file_or_exit!(TODO_FILE_NAME);
    let mut tasks = is_object_or_exit!(tasks, TODO_FILE_NAME);

    let task: &mut JsonValue = match &mut tasks[id] {
        JsonValue::Null => {
            print_error!("error: no task with given id [{}]", id);

            return;
        }
        task => task,
    };

    if let JsonValue::Short(s) = task[STATUS] {
        match s.as_str() {
            DONE => {
                print_error!("error: cannot discard a '{}' task", DONE);

                return;
            }
            DISCARDED => {
                print_error!("error: cannot discard a '{}' task", DISCARDED);

                return;
            }
            _ => (),
        }
    }

    task[STATUS] = JsonValue::String(String::from(DISCARDED));

    write_json_to_file_or_err!(tasks, TODO_FILE_NAME);
}
