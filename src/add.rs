use crate::constants::{DESCRIPTION, STATUS, TITLE, TODO_FILE_NAME, UNDONE, TMP_DIR, EDITOR};
use json;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::io::{self, Write};
use std::process::{self, Command};
use std::fs;

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
    let mut title = String::new();

    print!("enter task title: ");
    let _ = io::stdout().flush().unwrap_or_else(|_| {
            print_error!("error: cannot flush stdout");
            process::exit(1);        
    });
    
    match io::stdin().read_line(&mut title) {
        Ok(_) => (),
        Err(error) => {
            print_error!("error: {}", error);
            process::exit(1);
        }
    }

    let tmp_file = format!("{}/{}", TMP_DIR, hash(&title)); 
    let _ = Command::new(EDITOR)
        .arg(&tmp_file)
        .status()
        .unwrap_or_else(|err| {
            print_error!("error: {}", err);
            process::exit(1);
        });

    let title = title.trim();
    let description = fs::read_to_string(tmp_file).unwrap_or_else(|err| {
            print_error!("error: {}", err);
            process::exit(1);
    });
    let description = description.trim();

    add_full(title, description);
}

fn hash(string: &str) -> String {
    let mut s = DefaultHasher::new();

    string.hash(&mut s);

    let hash = s.finish();

    format!("{:x}", hash)
}
