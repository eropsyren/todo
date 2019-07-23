use crate::constants::{DISCARDED, DONE, MESSAGE, STATUS, SUBTASKS, TODO_FILE_NAME, UNDONE};
use colored::Colorize;
use json::{self, JsonValue};
use std::process;

pub fn list() {
    let tasks = get_json_from_file_or_return!(TODO_FILE_NAME);
    let tasks = is_object_or_return!(tasks, TODO_FILE_NAME);

    for (id, task) in tasks.entries() {
        println!("{}", format_task(id, task));

        for (id, sub_task) in task[SUBTASKS].entries() {
            println!("\t{}", format_task(id, sub_task));
        }
    }
}

pub fn format_task(id: &str, task: &JsonValue) -> String {
    let msg = task[MESSAGE].to_string();

    let status = match &task[STATUS] {
        JsonValue::Short(s) => s.as_str(),
        JsonValue::Null => {
            print_error!("error: missing {} property", STATUS);

            process::exit(1);
        }
        _ => {
            print_error!("error: property {} is not a json string", STATUS);

            process::exit(1);
        }
    };

    let msg = match status {
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
