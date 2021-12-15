use thiserror::Error;

#[derive(Error, Debug)]
pub enum ExecuteError {
    #[error("DecodeError: {0}")]
    DecodeError(String),
    #[error("ResultLengthError: unexpected result length expect {0}, got {1}")]
    ResultLengthError(usize, usize)
}

unsafe impl Sync for ExecuteError {}
unsafe impl Send for ExecuteError {}