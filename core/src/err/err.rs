use proc_macro2::Span;
use std::error::Error as StdError;

pub trait YukinoError: StdError {
    fn as_runtime_err(&self) -> RuntimeError {
        RuntimeError {
            msg: self.to_string(),
        }
    }

    fn as_cli_err(&self, pos: Span) -> CliError {
        CliError {
            msg: self.to_string(),
            pos,
        }
    }
}

pub struct RuntimeError {
    pub msg: String,
}

pub struct CliError {
    pub msg: String,
    pub pos: Span,
}
