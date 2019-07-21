use crate::constants::TODO_FILE_NAME;
use std::path::Path;
use std::fs;
use json::JsonValue;

macro_rules! print_error {
    ($($arg:tt)*) => {
        eprintln!("{}", colored::Colorize::red(std::fmt::format(format_args!($($arg)*)).as_str()));
    }
}

pub fn if_todo_exists(f: impl FnOnce() -> ()) {
    let exists = Path::new(TODO_FILE_NAME).exists();

    if exists {
        f();
    } else {
        print_error!("error: there is no .todo file");
    }
}

pub fn if_todo_not_exists(f: impl FnOnce() -> ()) {
    let exists = Path::new(TODO_FILE_NAME).exists();

    if exists {
        print_error!("error: .todo file exists already");
    } else {
        f();
    }
}

pub fn read_file_to_json<P: AsRef<Path>>(path: P) -> Result<JsonValue, String> {
    let file = match fs::read_to_string(path) {
        Ok(file) => file,
        Err(err) => {
            return Err(format!("{}", err))
        }
    };

    let json = match json::parse(&file) {
        Ok(json) => json,
        Err(err) => {
            return Err(format!("{}", err))
        }
    };

    Ok(json)
}
