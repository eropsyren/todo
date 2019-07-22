use crate::constants::{DONE, MESSAGE, TODO_FILE_NAME};
use colored::Colorize;
use json::{self, JsonValue};

pub fn list() {
    let tasks = get_json_from_file_or_return!(TODO_FILE_NAME);
    let tasks = validate_json_or_return!(tasks, TODO_FILE_NAME);

    for (id, task) in tasks.entries() {
        let id = format!("[{}]", id).cyan();
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
