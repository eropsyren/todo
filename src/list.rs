use crate::constants::{DISCARDED, DONE, STATUS, TITLE, TODO_FILE_NAME, UNDONE};
use colored::Colorize;
use json::{self, JsonValue};
use std::process;

pub fn list(is_id: bool) {
    let tasks = get_json_from_file_or_exit!(TODO_FILE_NAME);
    let tasks = is_object_or_exit!(tasks, TODO_FILE_NAME);

    for (id, task) in tasks.entries() {
        if is_id {
            let id = format!("[{}]", id);

            println!("{} {}", id.blue().bold(), format_task_title(task));
        } else {
            println!("{} {}", "-->".blue().bold(), format_task_title(task));
        }
    }
}

fn format_task_title(task: &JsonValue) -> String {
    let title = get_prop_or_exit!(task, TITLE, JsonValue::Short, JsonValue::String).to_string();
    let status = get_prop_or_exit!(task, STATUS, JsonValue::Short).to_string();

    let title = match status.as_str() {
        DONE => title.bright_green().bold(),
        UNDONE => title.bright_yellow().bold(),
        DISCARDED => title.bright_red().bold(),
        status => {
            print_error!("error: {} is an indvalid status", status);

            process::exit(1);
        }
    };

    format!("{}", title)
}
