use json::object::Object;
use json::JsonValue;
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

impl TryFrom<Object> for Task {
    type Error = ParseError;

    fn try_from(mut obj: Object) -> Result<Self, Self::Error> {
        match obj.remove("msg") {
            Some(JsonValue::String(msg)) => Ok(Task { msg }),
            _ => Err(ParseError::new(obj)),
        }
    }
}

#[derive(Debug)]
pub struct ParseError {
    obj: Object,
}

impl ParseError {
    pub fn new(obj: Object) -> Self {
        ParseError { obj }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "unable to parse as .json: {}", self.obj.pretty(4))
    }
}

impl Error for ParseError {}
