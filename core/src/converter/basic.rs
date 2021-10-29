use crate::db::ty::{DatabaseType, DatabaseValue, ValuePack};
use crate::err::DataConvertError;
use crate::err::RuntimeResult;
use crate::err::YukinoError;
use crate::interface::converter::DataConverter;
use iroha::ToTokens;

macro_rules! converter_of {
    ($field_type:ty, $name:ident, $enum_variant:ident) => {
        #[derive(ToTokens, Clone)]
        #[Iroha(mod_path = "yukino::converter::basic")]
        pub struct $name {
            column_name: String,
        }

        impl DataConverter for $name {
            type FieldType = $field_type;
            fn field_value_converter(
                &self,
            ) -> Box<dyn Fn(&ValuePack) -> RuntimeResult<Self::FieldType>> {
                let column_name = self.column_name.clone();
                Box::new(move |values| {
                    values
                        .get(column_name.as_str())
                        .map(|data| match data {
                            DatabaseValue::$enum_variant(data) => Ok(data.clone()),
                            _ => Err(DataConvertError::UnexpectedValueType(column_name.clone())
                                .as_runtime_err()),
                        })
                        .ok_or_else(|| {
                            DataConvertError::ColumnDataNotFound(column_name.clone())
                                .as_runtime_err()
                        })?
                })
            }

            fn is_null(&self, values: &ValuePack) -> bool {
                values
                    .get(self.column_name.as_str())
                    .map(|data| matches!(data, DatabaseValue::Null(DatabaseType::$enum_variant)))
                    .unwrap_or(false)
            }

            fn to_database_values_by_ref(
                &self,
                value: &Self::FieldType,
            ) -> RuntimeResult<ValuePack> {
                Ok([(
                    self.column_name.clone(),
                    DatabaseValue::$enum_variant(value.clone()),
                )]
                .into_iter()
                .collect())
            }

            fn get_columns(&self) -> Vec<String> {
                vec![self.column_name.clone()]
            }
        }
    };
}

converter_of!(i16, ShortDataConverter, SmallInteger);
converter_of!(u16, UnsignedShortDataConverter, UnsignedSmallInteger);
converter_of!(i32, IntDataConverter, Integer);
converter_of!(u32, UnsignedIntDataConverter, UnsignedInteger);
converter_of!(i64, LongDataConverter, BigInteger);
converter_of!(u64, UnsignedLongDataConverter, UnsignedBigInteger);
converter_of!(f32, FloatDataConverter, Float);
converter_of!(f64, DoubleDataConverter, Double);
converter_of!(String, StringDataConverter, String);
converter_of!(char, CharDataConverter, Character);
