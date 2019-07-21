use crate::constants::{DONE, ID, MESSAGE, TODO_FILE_NAME};
use json::{self, JsonValue};
use std::collections::hash_map::DefaultHasher;
use std::fs;
use std::fs::File;
use std::hash::{Hash, Hasher};

pub fn add(task: &str) {
    let file = match fs::read_to_string(TODO_FILE_NAME) {
        Ok(file) => file,
        Err(err) => {
            print_error!("error reading file {}: {}", TODO_FILE_NAME, err);

            return;
        }
    };

    let tasks = match json::parse(&file) {
        Ok(json) => json,
        Err(err) => {
            print_error!("error parsing file {} as json: {}", TODO_FILE_NAME, err);

            return;
        }
    };

    let mut tasks = match tasks {
        JsonValue::Array(_) => tasks,
        _ => {
            print_error!("file {} is not a json array", TODO_FILE_NAME);

            return;
        }
    };

    let new_task = json::object! {
        ID => hash(task),
        MESSAGE => task,
        DONE => false,
    };

    match tasks.push(new_task) {
        Ok(_) => (),
        Err(err) => {
            print_error!("error pushing to json array: {}", err);

            return;
        }
    }

    match File::create(TODO_FILE_NAME) {
        Ok(mut file) => match tasks.write(&mut file) {
            Ok(_) => (),
            Err(err) => print_error!("error writing to {}: {}", TODO_FILE_NAME, err),
        },
        Err(err) => print_error!("error rewriting {}: {}", TODO_FILE_NAME, err),
    }
}

fn hash(string: &str) -> u64 {
    let mut s = DefaultHasher::new();

    string.hash(&mut s);
    s.finish()
}
