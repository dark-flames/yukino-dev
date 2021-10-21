use crate::db::ty::ValuePack;
use crate::err::YukinoError;
use quote::ToTokens;
use std::fmt::Debug;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DataConvertError {
    #[error("ColumnDataNotFound: data of column `{0}` was not found in value pack")]
    ColumnDataNotFound(String),
    #[error("UnexpectedValueType: Unexpected data type of column `{0}`")]
    UnexpectedValueType(String),
}

impl YukinoError for DataConvertError {}

pub trait DataConverter: ToTokens {
    type FieldType;
    fn to_field_value(&self, values: &ValuePack) -> Result<Self::FieldType, DataConvertError>;

    fn to_database_values(&self, value: Self::FieldType) -> Result<ValuePack, DataConvertError> {
        self.to_database_values_by_ref(&value)
    }

    fn to_database_values_by_ref(
        &self,
        value: &Self::FieldType,
    ) -> Result<ValuePack, DataConvertError>;
}
