use proc_macro2::Span;
use std::error::Error as StdError;
use std::fmt::{Debug, Display, Formatter};

pub trait YukinoError: StdError {
    fn as_runtime_err(&self) -> RuntimeError {
        RuntimeError {
            msg: self.to_string(),
        }
    }

    fn as_cli_err(&self, pos: Option<Span>) -> CliError {
        CliError {
            msg: self.to_string(),
            pos,
        }
    }
}

#[derive(Debug)]
pub struct RuntimeError {
    pub msg: String,
}

#[derive(Debug)]
pub struct CliError {
    pub msg: String,
    pub pos: Option<Span>,
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl StdError for RuntimeError {}

impl Display for CliError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl StdError for CliError {}