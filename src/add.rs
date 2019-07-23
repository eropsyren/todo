use crate::constants::{MESSAGE, STATUS, SUBTASKS, TODO_FILE_NAME, UNDONE};
use json::{self, JsonValue};
use std::collections::hash_map::DefaultHasher;
use std::fs::File;
use std::hash::{Hash, Hasher};

pub fn add(task: &str) {
    let tasks = get_json_from_file_or_return!(TODO_FILE_NAME);
    let mut tasks = validate_json_or_return!(tasks, TODO_FILE_NAME);
    let task = task.trim();
    let task_hash = hash(task);

    if tasks.has_key(&task_hash) {
        print_error!("error: task '{}' already present", task);

        return;
    }

    tasks[task_hash] = json::object! {
        MESSAGE => task,
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

pub fn add_subtask(id: &str, task: &str) {
    let tasks = get_json_from_file_or_return!(TODO_FILE_NAME);
    let mut tasks = validate_json_or_return!(tasks, TODO_FILE_NAME);
    let super_task = get_prop_or_return!(tasks, id, JsonValue::Object);
    let super_task_msg = get_prop_or_return!(super_task, MESSAGE, JsonValue::Short).to_string();
    let task = task.trim();
    let task_hash = hash(&format!("{}\n{}", super_task_msg, task));

    if !super_task.has_key(SUBTASKS) {
        super_task[SUBTASKS] = json::object! {};
    }

    let sub_tasks = get_prop_or_return!(super_task, SUBTASKS, JsonValue::Object);

    if sub_tasks.has_key(&task_hash) {
        print_error!("error: subtask '{}' is present already", task);

        return;
    }

    sub_tasks[task_hash] = json::object! {
        MESSAGE => task,
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
