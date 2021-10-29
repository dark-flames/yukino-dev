use quote::ToTokens;

use crate::db::ty::ValuePack;
use crate::err::{DataConvertError, RuntimeResult, YukinoError};

pub trait DataConverter: ToTokens {
    type Output: 'static;
    fn nullable_field_value_converter(
        &self,
    ) -> Box<dyn Fn(&ValuePack) -> RuntimeResult<Option<Self::Output>>>;

    fn field_value_converter(&self) -> Box<dyn Fn(&ValuePack) -> RuntimeResult<Self::Output>> {
        let converter = self.nullable_field_value_converter();
        let columns: String = self.get_columns().join(", ");

        Box::new(move |v| {
            (*converter)(v)?
                .ok_or_else(|| DataConvertError::GotNullOnNotNullField(columns.clone()).as_runtime_err())
        })
    }

    fn to_database_values(&self, value: Self::Output) -> RuntimeResult<ValuePack> {
        self.to_database_values_by_ref(&value)
    }

    fn to_database_values_by_ref(&self, value: &Self::Output) -> RuntimeResult<ValuePack>;

    fn get_columns(&self) -> Vec<String>;
}
