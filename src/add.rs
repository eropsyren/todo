use crate::constants::{DESCRIPTION, EDITOR, STATUS, TITLE, TMP_DIR, TODO_FILE_NAME, UNDONE};
use json;
use std::collections::hash_map::DefaultHasher;
use std::fs::{self, File};
use std::hash::{Hash, Hasher};
use std::process::{self, Command};

pub fn add_full(task_title: &str, task_description: &str) {
    let tasks = get_json_from_file_or_exit!(TODO_FILE_NAME);
    let mut tasks = is_object_or_exit!(tasks, TODO_FILE_NAME);
    let task = task_title.trim();
    let task_hash = hash(task);

    if tasks.has_key(&task_hash) {
        print_error!("error: task '{}' already present", task);

        return;
    }

    tasks[task_hash] = json::object! {
        TITLE => task_title,
        DESCRIPTION => task_description,
        STATUS => UNDONE,
    };

    write_json_to_file_or_err!(tasks, TODO_FILE_NAME);
}

pub fn add_title(task_title: &str) {
    add_full(task_title, "");
}

pub fn add_with_prompt() {
    let tmp_file = format!("{}/new_task", TMP_DIR);

    File::create(&tmp_file).unwrap_or_else(|err| {
        print_error!("error: {}", err);
        process::exit(1);
    });

    let _ = Command::new(EDITOR)
        .arg(&tmp_file)
        .status()
        .unwrap_or_else(|err| {
            print_error!("error: {}", err);
            process::exit(1);
        });

    let file_content = fs::read_to_string(&tmp_file).unwrap_or_else(|err| {
        print_error!("error: {}", err);
        process::exit(1);
    });

    let file_lines: Vec<&str> = file_content.trim().lines().collect();
    let title = match file_lines.first() {
        Some(title) => title,
        None => {
            print_error!("error: no title provided");
            process::exit(1);
        }
    };
    let title = title.trim();
    let description = &file_lines[1..].join("\n");

    add_full(title, description);
}

fn hash(string: &str) -> String {
    let mut s = DefaultHasher::new();

    string.hash(&mut s);

    let hash = s.finish();

    to_base_36(hash)
}

fn to_base_36(mut value: u64) -> String {
    const BUFFER: [char; 36] = [
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h',
        'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ];

    let mut result = String::new();

    while value / 36 != 0 {
        result.push(BUFFER[(value % 36) as usize]);
        value = value / 36;
    }

    result.push(BUFFER[(value % 36) as usize]);

    result.chars().rev().collect()
}
