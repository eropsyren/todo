use crate::constants::{DESCRIPTION, EDITOR, TMP_DIR, TODO_FILE_NAME};
use json::JsonValue;
use std::fs;
use std::process::{self, Command};

pub fn edit(id: &str) {
    let tasks = get_json_from_file_or_exit!(TODO_FILE_NAME);
    let mut tasks = is_object_or_exit!(tasks, TODO_FILE_NAME);
    let task = get_mut_prop_or_exit!(tasks, id, JsonValue::Object);
    let old_description = get_prop_or_exit!(
        task,
        DESCRIPTION,
        JsonValue::String,
        JsonValue::Short,
        JsonValue::Object
    )
    .to_string();

    let tmp_file = format!("{}/{}", TMP_DIR, id);

    // make sure that the file to be edited has old description in it
    fs::write(&tmp_file, old_description).unwrap_or_else(|err| {
        print_error!("error: {}", err);
        process::exit(1);
    });

    let _ = Command::new(EDITOR)
        .arg(&tmp_file)
        .status()
        .unwrap_or_else(|err| {
            print_error!("error: {}", err);
            process::exit(1);
        });

    let new_description = fs::read_to_string(tmp_file).unwrap_or_else(|err| {
        print_error!("error: {}", err);
        process::exit(1);
    });
    let new_description = new_description.trim();

    if task.has_key(DESCRIPTION) {
        task[DESCRIPTION] = JsonValue::from(new_description);
        write_json_to_file_or_err!(tasks, TODO_FILE_NAME);
    } else {
        print_error!("error: missing property {} in task [{}]", DESCRIPTION, id);
        process::exit(1);
    }
}
