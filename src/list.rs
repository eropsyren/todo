use crate::constants::TODO_FILE_NAME;
use colored::Colorize;
use std::error::Error;
use std::fs;

pub fn list() {
    let content = fs::read_to_string(TODO_FILE_NAME);

    match content {
        Ok(content) => {
            for (idx, line) in content.lines().enumerate() {
                let idx = format!("[{}]", idx);
                println!("{} {}", idx.cyan(), line.yellow());
            }
        }
        Err(err) => eprintln!(
            "error reading file {}: {}",
            TODO_FILE_NAME,
            err.description()
        ),
    }
}
