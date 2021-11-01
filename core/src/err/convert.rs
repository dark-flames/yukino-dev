use thiserror::Error;

use crate::err::YukinoError;

#[derive(Error, Debug)]
pub enum DataConvertError {
    #[error("DataNotFound: data of alias `{0}` was not found in value pack")]
    DataNotFound(String),
    #[error("UnexpectedValueType: Unexpected data type of column `{0}`")]
    UnexpectedValueType(String),
    #[error("GotNullOnNotNullField: Got Null on not null field by columns `{0}`")]
    GotNullOnNotNullField(String),
    #[error("UnmatchedParameterCount: Unmatched parameters count when resolving `{0}`, expect `{1}`, got `{2}`")]
    UnmatchedParameterCount(String, usize, usize),
}

impl YukinoError for DataConvertError {}
