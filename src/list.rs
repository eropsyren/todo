use crate::constants::{DISCARDED, DONE, MESSAGE, STATUS, TODO_FILE_NAME, UNDONE};
use colored::Colorize;
use json::{self, JsonValue};

pub fn list() {
    let tasks = get_json_from_file_or_return!(TODO_FILE_NAME);
    let tasks = validate_json_or_return!(tasks, TODO_FILE_NAME);

    for (id, task) in tasks.entries() {
        let id = format!("[{}]", id).cyan();
        let msg = task[MESSAGE].to_string();

        let status = match &task[STATUS] {
            JsonValue::Short(s) => s.as_str(),
            JsonValue::Null => {
                print_error!("error: missing {} property", STATUS);

                return;
            }
            _ => {
                print_error!("error: property {} is not a json string", STATUS);
                
                return;
            }
        };

        let msg = match status {
            DONE => msg.green(),
            UNDONE => msg.yellow(),
            DISCARDED => msg.red(),
            status => {
                print_error!("error: {} is an indvalid status", status);
                
                return;
            }
        };

        println!("{} {}", id, msg);
    }
}
