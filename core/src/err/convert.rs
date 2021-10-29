use thiserror::Error;

use crate::err::YukinoError;

#[derive(Error, Debug)]
pub enum DataConvertError {
    #[error("ColumnDataNotFound: data of column `{0}` was not found in value pack")]
    ColumnDataNotFound(String),
    #[error("UnexpectedValueType: Unexpected data type of column `{0}`")]
    UnexpectedValueType(String),
    #[error("GotNullOnNotNullField: Got Null on not null field by columns `{0}`")]
    GotNullOnNotNullField(String),
}

impl YukinoError for DataConvertError {}
