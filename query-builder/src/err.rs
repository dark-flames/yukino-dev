use thiserror::Error;

#[derive(Error, Debug)]
pub enum ExecuteError {
    #[error("DecodeError: {0}")]
    DecodeError(String)
}

unsafe impl Sync for ExecuteError {}
unsafe impl Send for ExecuteError {}