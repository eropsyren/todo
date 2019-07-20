use json;
use json::JsonValue;
use std::convert::Into;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt;

pub struct Task {
    msg: String,
}

impl Task {
    pub fn new(msg: String) -> Self {
        Task { msg }
    }
}

impl TryFrom<JsonValue> for Task {
    type Error = TaskError;

    fn try_from(json_value: JsonValue) -> Result<Self, Self::Error> {
        match json_value {
            JsonValue::Object(mut obj) => match obj.remove("msg") {
                Some(JsonValue::String(msg)) => Ok(Task::new(msg)),
                _ => Err(TaskError::MissingProperty("msg")),
            },
            _ => Err(TaskError::NotAnObject),
        }
    }
}

impl Into<JsonValue> for Task {
    fn into(self) -> JsonValue {
        json::object! {
            "msg" => self.msg,
        }
    }
}

#[derive(Debug)]
pub enum TaskError {
    MissingProperty(&'static str),
    NotAnObject,
}

impl fmt::Display for TaskError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TaskError::MissingProperty(prop_name) => {
                write!(f, "error: missing property name {}", prop_name)
            }
            TaskError::NotAnObject => write!(f, "error: not a json object"),
        }
    }
}

impl Error for TaskError {}
