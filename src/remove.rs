use crate::constants::{TODO_FILE_NAME};

pub fn remove(id: &str) {
    let tasks = get_json_from_file_or_exit!(TODO_FILE_NAME);
    let mut tasks = is_object_or_exit!(tasks, TODO_FILE_NAME);

    tasks.remove(id);

    write_json_to_file_or_err!(tasks, TODO_FILE_NAME);
}
