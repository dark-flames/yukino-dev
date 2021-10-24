use thiserror::Error;

use crate::err::YukinoError;

#[derive(Error, Debug)]
pub enum DataConvertError {
    #[error("ColumnDataNotFound: data of column `{0}` was not found in value pack")]
    ColumnDataNotFound(String),
    #[error("UnexpectedValueType: Unexpected data type of column `{0}`")]
    UnexpectedValueType(String),
}

impl YukinoError for DataConvertError {}
