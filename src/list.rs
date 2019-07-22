use crate::constants::{DONE, ID, MESSAGE, TODO_FILE_NAME};
use colored::Colorize;
use json::{self, JsonValue};

pub fn list() {
    let tasks = get_json_from_file_or_return!(TODO_FILE_NAME);
    let tasks = validate_json_get_vec_or_return!(tasks, TODO_FILE_NAME);

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
