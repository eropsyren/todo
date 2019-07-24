use crate::constants::{STATUS, TITLE, TODO_FILE_NAME, UNDONE};
use json;
use std::collections::hash_map::DefaultHasher;
use std::fs::File;
use std::hash::{Hash, Hasher};

pub fn add(task: &str) {
    let tasks = get_json_from_file_or_exit!(TODO_FILE_NAME);
    let mut tasks = is_object_or_exit!(tasks, TODO_FILE_NAME);
    let task = task.trim();
    let task_hash = hash(task);

    if tasks.has_key(&task_hash) {
        print_error!("error: task '{}' already present", task);

        return;
    }

    tasks[task_hash] = json::object! {
        TITLE => task,
        STATUS => UNDONE,
    };

    match File::create(TODO_FILE_NAME) {
        Ok(mut file) => match tasks.write(&mut file) {
            Ok(_) => (),
            Err(err) => print_error!("error writing to {}: {}", TODO_FILE_NAME, err),
        },
        Err(err) => print_error!("error rewriting {}: {}", TODO_FILE_NAME, err),
    }
}

fn hash(string: &str) -> String {
    let mut s = DefaultHasher::new();

    string.hash(&mut s);

    let hash = s.finish();

    format!("{:x}", hash)
}
