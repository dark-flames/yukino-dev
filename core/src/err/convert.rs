use thiserror::Error;

use crate::err::YukinoError;

#[derive(Error, Debug)]
pub enum ConvertError {
    #[error("UnexpectedValueType: The type of input is unexpected")]
    UnexpectedValueType,
}

impl YukinoError for ConvertError {}
