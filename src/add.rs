use crate::constants::TODO_FILE_NAME;
use json::{self, JsonValue};
use std::fs;
use std::fs::File;

pub fn add(task: &str) {
    let file = match fs::read_to_string(TODO_FILE_NAME) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("error reading file {}: {}", TODO_FILE_NAME, err);

            return;
        }
    };

    let tasks = match json::parse(&file) {
        Ok(json) => json,
        Err(err) => {
            eprintln!("error parsing file {} as json: {}", TODO_FILE_NAME, err);

            return;
        }
    };

    let mut tasks = match tasks {
        JsonValue::Array(_) => tasks,
        _ => {
            eprintln!("file {} is not a json array", TODO_FILE_NAME);

            return;
        }
    };

    let new_task = json::object! {
        "msg" => task,
    };

    match tasks.push(new_task) {
        Ok(_) => (),
        Err(err) => {
            eprintln!("error pushing to json array: {}", err);

            return;
        }
    }

    match File::create(TODO_FILE_NAME) {
        Ok(mut file) => match tasks.write(&mut file) {
            Ok(_) => (),
            Err(err) => eprintln!("error writing to {}: {}", TODO_FILE_NAME, err),
        },
        Err(err) => eprintln!("error rewriting {}: {}", TODO_FILE_NAME, err),
    }
}
