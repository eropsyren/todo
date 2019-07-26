use crate::constants::{DESCRIPTION, DISCARDED, DONE, STATUS, TITLE, TODO_FILE_NAME, UNDONE};
use colored::Colorize;
use json::{self, JsonValue};
use std::process;

pub fn list(is_long: bool, filter: Option<&str>) {
    let filter = validate_filter(filter);
    let tasks = get_json_from_file_or_exit!(TODO_FILE_NAME);
    let tasks = is_object_or_exit!(tasks, TODO_FILE_NAME);

    for (id, task) in tasks.entries() {
        let task_status =
            get_prop_or_exit!(task, STATUS, JsonValue::Short, JsonValue::String).to_string();

        if matches_filter(task_status.as_str(), filter) {
            if is_long {
                let id = format!("[{}]", id);

                println!("{} {}", id.blue().bold(), format_task_title(task));
            } else {
                println!("{} {}", "-->".blue().bold(), format_task_title(task));
            }
        }
    }
}

pub fn list_task(id: &str) {
    let tasks = get_json_from_file_or_exit!(TODO_FILE_NAME);
    let tasks = is_object_or_exit!(tasks, TODO_FILE_NAME);
    let task = get_prop_or_exit!(tasks, id, JsonValue::Object);
    let id = format!("[{}]", id);
    let description =
        get_prop_or_exit!(task, DESCRIPTION, JsonValue::Short, JsonValue::String).to_string();
    let description: String = description
        .lines()
        .map(|line| format!("\t{}", line.bold()))
        .collect::<Vec<String>>()
        .join("\n");

    println!("{} {}", id.blue().bold(), format_task_title(task));
    println!("{}", description);
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

fn validate_filter(filter: Option<&str>) -> Option<&str> {
    match filter {
        Some(val) => match val {
            DONE | UNDONE | DISCARDED => Some(val),
            indvalid => {
                print_error!("error: '{}' is an invalid filter", indvalid);
                process::exit(1);
            }
        },
        None => None,
    }
}

fn matches_filter(status: &str, filter: Option<&str>) -> bool {
    match filter {
        Some(val) => val == status,
        None => true,
    }
}
