use crate::constants::TODO_FILE_NAME;
use json::JsonValue;
use std::fs;
use std::path::Path;

macro_rules! print_error {
    ($($arg:tt)*) => {
        eprintln!("{}", colored::Colorize::bold(colored::Colorize::red(std::fmt::format(format_args!($($arg)*)).as_str())));
    }
}

macro_rules! get_json_from_file_or_exit {
    ($path:expr) => {
        match crate::utils::read_file_to_json($path) {
            Ok(json) => json,
            Err(err) => {
                print_error!("error reading {} file into json: {}", $path, err);
                std::process::exit(1);
            }
        };
    };
}

macro_rules! is_object_or_exit {
    ($json:expr, $path:expr) => {
        match $json {
            json::JsonValue::Object(_) => $json,
            _ => {
                print_error!("error: file {} is not a json object", $path);
                std::process::exit(1);
            }
        }
    };
}

macro_rules! write_json_to_file_or_err {
    ($json:expr, $path:expr) => {
        match std::fs::File::create($path) {
            Ok(mut file) => match $json.write(&mut file) {
                Ok(_) => (),
                Err(err) => print_error!("error writing to {}: {}", $path, err),
            },
            Err(err) => print_error!("error rewriting {}: {}", $path, err),
        }
    };
}

macro_rules! get_prop_or_exit {
    ($json_val:expr, $prop_name:expr, $(JsonValue::$required_type:ident), *) => {
        match &$json_val[$prop_name] {
            $(
                obj @ JsonValue::$required_type(_) => obj,
            )*
            JsonValue::Null => {
                print_error!("error: missing property '{}'", $prop_name);
                std::process::exit(1);
            }
            _ => {
                print_error!(
                    "error: the value associated with '{}' has an incorrect type",
                    $prop_name
                );
                std::process::exit(1);
            }
        };
    };
}

macro_rules! get_mut_prop_or_exit {
    ($json_val:expr, $prop_name:expr, $(JsonValue::$required_type:ident), *) => {
        match &mut $json_val[$prop_name] {
            $(
                obj @ JsonValue::$required_type(_) => obj,
            )*
            JsonValue::Null => {
                print_error!("error: missing property {}", $prop_name);
                std::process::exit(1);
            }
            _ => {
                print_error!(
                    "error: the value associated with '{}' has an incorrect type",
                    $prop_name
                );
                std::process::exit(1);
            }
        };
    };
}

pub fn if_todo_exists(f: impl FnOnce() -> ()) {
    let exists = Path::new(TODO_FILE_NAME).exists();

    if exists {
        f();
    } else {
        print_error!("error: there is no {} file", TODO_FILE_NAME);
    }
}

pub fn if_todo_not_exists(f: impl FnOnce() -> ()) {
    let exists = Path::new(TODO_FILE_NAME).exists();

    if exists {
        print_error!("error: {} file exists already", TODO_FILE_NAME);
    } else {
        f();
    }
}

pub fn read_file_to_json<P: AsRef<Path>>(path: P) -> Result<JsonValue, String> {
    let file = match fs::read_to_string(path) {
        Ok(file) => file,
        Err(err) => return Err(format!("{}", err)),
    };

    let json = match json::parse(&file) {
        Ok(json) => json,
        Err(err) => return Err(format!("{}", err)),
    };

    Ok(json)
}
