use crate::constants::{MESSAGE, STATUS, SUBTASKS, TODO_FILE_NAME, UNDONE};
use json::{self, JsonValue};
use std::collections::hash_map::DefaultHasher;
use std::fs::File;
use std::hash::{Hash, Hasher};

pub fn add(task: &str) {
    let task = task.trim();
    let tasks = get_json_from_file_or_return!(TODO_FILE_NAME);
    let mut tasks = validate_json_or_return!(tasks, TODO_FILE_NAME);
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
    let task = task.trim();
    let tasks = get_json_from_file_or_return!(TODO_FILE_NAME);
    let mut tasks = validate_json_or_return!(tasks, TODO_FILE_NAME);

    let super_task: &mut JsonValue = match &mut tasks[id] {
        obj @ JsonValue::Object(_) => obj,
        JsonValue::Null => {
            print_error!("error: no task with {} id", id);

            return;
        }
        _ => {
            print_error!("error: the task with {} id is not a json object", id);

            return;
        }
    };

    if !super_task.has_key(SUBTASKS) {
        super_task[SUBTASKS] = json::object! {};
    }

    let sub_tasks = match &mut super_task[SUBTASKS] {
        obj @ JsonValue::Object(_) => obj,
        JsonValue::Null => unreachable!(),
        _ => {
            print_error!("error: {} property is not a json object", SUBTASKS);

            return;
        }
    };

    let task_hash = hash(task);

    if sub_tasks.has_key(&task_hash) {
        print_error!("error: subtask '{}' already present", task);

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
    s.finish().to_string()
}
