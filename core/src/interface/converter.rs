use quote::ToTokens;

use crate::db::ty::ValuePack;
use crate::err::RuntimeResult;

pub trait DataConverter: ToTokens {
    type FieldType;
    fn field_value_converter(&self) -> Box<dyn Fn(&ValuePack) -> RuntimeResult<Self::FieldType>>;

    fn to_database_values(&self, value: Self::FieldType) -> RuntimeResult<ValuePack> {
        self.to_database_values_by_ref(&value)
    }

    fn to_database_values_by_ref(&self, value: &Self::FieldType) -> RuntimeResult<ValuePack>;

    fn get_columns(&self) -> Vec<String>;
}
