use crate::constants::{DISCARDED, DONE, MESSAGE, STATUS, TODO_FILE_NAME, UNDONE};
use colored::Colorize;
use json::{self, JsonValue};
use std::process;

pub fn list() {
    let tasks = get_json_from_file_or_exit!(TODO_FILE_NAME);
    let tasks = is_object_or_exit!(tasks, TODO_FILE_NAME);

    for (id, task) in tasks.entries() {
        println!("{}", format_task(id, task));
    }
}

pub fn format_task(id: &str, task: &JsonValue) -> String {
    let msg = get_prop_or_exit!(task, MESSAGE, JsonValue::Short, JsonValue::String).to_string();
    let status = get_prop_or_exit!(task, STATUS, JsonValue::Short).to_string();

    let msg = match status.as_str() {
        DONE => msg.bright_green().bold(),
        UNDONE => msg.bright_yellow().bold(),
        DISCARDED => msg.bright_red().bold(),
        status => {
            print_error!("error: {} is an indvalid status", status);

            process::exit(1);
        }
    };

    let id = format!("[{}]", id).cyan();

    format!("{} {}", id, msg)
}
