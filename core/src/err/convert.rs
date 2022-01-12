use thiserror::Error;

use interface::DatabaseType;

use crate::err::YukinoError;

#[derive(Error, Debug)]
pub enum ConvertError {
    #[error("UnexpectedValueType: The type of input is unexpected, expect `{0}` got `{1}`")]
    UnexpectedValueType(DatabaseType, DatabaseType),
}

impl YukinoError for ConvertError {}
