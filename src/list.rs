use crate::constants::{DONE, ID, MESSAGE, TODO_FILE_NAME};
use crate::utils;
use colored::Colorize;
use json::{self, JsonValue};

pub fn list() {
    let tasks = match utils::read_file_to_json(TODO_FILE_NAME) {
        Ok(json) => json,
        Err(err) => {
            print_error!("error reading {} file into json: {}", TODO_FILE_NAME, err);

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
        let id = format!("[{}]", task[ID]).cyan();
        let msg = task[MESSAGE].to_string();
        let msg = match task[DONE] {
            JsonValue::Boolean(done) => {
                if done {
                    msg.green()
                } else {
                    msg.yellow()
                }
            }
            _ => {
                print_error!("error: {} property is not a boolean", DONE);
                return;
            }
        };

        println!("{} {}", id, msg);
    }
}
