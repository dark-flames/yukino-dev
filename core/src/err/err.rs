use std::error::Error as StdError;

pub trait YukinoError: StdError {
    fn as_runtime_err(&self) -> RuntimeError {
        RuntimeError {
            msg: self.to_string(),
        }
    }
}

pub struct RuntimeError {
    pub msg: String,
}
