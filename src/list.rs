use crate::constants::TODO_FILE_NAME;
use colored::Colorize;
use json::{self, JsonValue};
use std::fs;

pub fn list() {
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

    let tasks = match tasks {
        JsonValue::Array(tasks) => tasks,
        _ => {
            print_error!("error parsing file {}: not a json array", TODO_FILE_NAME);

            return;
        }
    };

    for task in tasks {
        let id = format!("[{}]", task["id"]);
        let msg = task["msg"].to_string();

        println!("{} {}", id.cyan(), msg.yellow());
    }
}
