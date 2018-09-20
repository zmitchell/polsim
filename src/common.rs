use std::error;
use std::fmt;
use std::result;

pub type Result<T> = result::Result<T, PolSimError>;

#[derive(Debug)]
pub enum PolSimError {
    NoBeam,
    NoElements,
}

impl error::Error for PolSimError {
    fn description(&self) -> &str {
        ""
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

impl fmt::Display for PolSimError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::PolSimError::*;
        match self {
            NoBeam => write!(f, "NoBeam: A simulation requires a beam."),
            NoElements => write!(f, "NoElements: A simulation requires at least one optical element"),
        }
    }
}