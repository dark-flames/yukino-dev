use crate::err::query::ErrorOnExpr;
use crate::err::view::ErrorOnView;
use thiserror::Error;

use crate::err::YukinoError;

#[derive(Error, Debug)]
pub enum ConvertError {
    #[error("UnexpectedValueType: The type of input is unexpected")]
    UnexpectedValueType,
    #[error("UnexpectedValueCount: The input value count is unexpected")]
    UnexpectedValueCount,
}

impl YukinoError for ConvertError {}

impl ErrorOnExpr for ConvertError {}

impl ErrorOnView for ConvertError {}
