use crate::constants::TODO_FILE_NAME;
use std::path::Path;

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
