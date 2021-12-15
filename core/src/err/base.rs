use std::error::Error as StdError;
use std::fmt::{Debug, Display, Formatter};

pub type RuntimeResult<T> = Result<T, RuntimeError>;

pub trait YukinoError: StdError {
    fn as_runtime_err(&self) -> RuntimeError {
        RuntimeError {
            msg: self.to_string(),
        }
    }
}

#[derive(Debug)]
pub struct RuntimeError {
    pub msg: String,
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl StdError for RuntimeError {}

