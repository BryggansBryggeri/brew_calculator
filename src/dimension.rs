use std::error as std_error;

// TODO: Crate only access
pub trait Dimension {
    fn value(self) -> f32;
}

pub trait DimensionLess {}

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    ValueError(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::ValueError(description) => write!(f, "{}", description),
        }
    }
}

impl std_error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::ValueError(_) => "ValueError",
        }
    }

    fn cause(&self) -> Option<&dyn std_error::Error> {
        None
    }
}
