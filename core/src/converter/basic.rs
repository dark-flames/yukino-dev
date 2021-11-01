use crate::converter::Converter;
use crate::db::ty::{DatabaseType, DatabaseValue};
use crate::err::RuntimeResult;
use iroha::ToTokens;

macro_rules! basic_ty_converter {
    ($field_type:ty, $name:ident, $enum_variant:ident) => {
        #[derive(ToTokens, Clone)]
        #[Iroha(mod_path = "yukino::converter::basic")]
        pub struct $name();

        unsafe impl Sync for $name {}

        impl Converter for $name {
            type Output = $field_type;

            fn deserializer(
                &self,
            ) -> Box<dyn Fn(&[&DatabaseValue]) -> RuntimeResult<Self::Output>> {
                Box::new(|v| {
                    v.first()
                        .map(|value| {
                            if let DatabaseValue::$enum_variant(nested) = value {
                                Ok(nested.clone())
                            } else {
                                panic!("error handle");
                            }
                        })
                        .ok_or_else(|| panic!("error handle"))?
                })
            }

            fn serialize(&self, value: &Self::Output) -> RuntimeResult<Vec<DatabaseValue>> {
                Ok(vec![DatabaseValue::$enum_variant(value.clone())])
            }
        }
    };
}

macro_rules! optional_basic_ty_converter {
    ($field_type:ty, $name:ident, $enum_variant:ident) => {
        #[derive(ToTokens, Clone)]
        #[Iroha(mod_path = "yukino::converter::basic")]
        pub struct $name();

        unsafe impl Sync for $name {}

        impl Converter for $name {
            type Output = Option<$field_type>;

            fn deserializer(
                &self,
            ) -> Box<dyn Fn(&[&DatabaseValue]) -> RuntimeResult<Self::Output>> {
                Box::new(|v| {
                    v.first()
                        .map(|value| match value {
                            DatabaseValue::$enum_variant(nested) => Ok(Some(nested.clone())),
                            DatabaseValue::Null(DatabaseType::$enum_variant) => Ok(None),
                            _ => {
                                panic!("error handle")
                            }
                        })
                        .ok_or_else(|| panic!("error handle"))?
                })
            }

            fn serialize(&self, value: &Self::Output) -> RuntimeResult<Vec<DatabaseValue>> {
                if let Some(nested) = value {
                    Ok(vec![DatabaseValue::$enum_variant(nested.clone())])
                } else {
                    Ok(vec![DatabaseValue::Null(DatabaseType::$enum_variant)])
                }
            }
        }
    };
}

basic_ty_converter!(i16, ShortConverter, SmallInteger);
basic_ty_converter!(u16, UnsignedShortConverter, UnsignedSmallInteger);
basic_ty_converter!(i32, IntConverter, Integer);
basic_ty_converter!(u32, UnsignedIntConverter, UnsignedInteger);
basic_ty_converter!(i64, LongConverter, BigInteger);
basic_ty_converter!(u64, UnsignedLongConverter, UnsignedBigInteger);
basic_ty_converter!(f32, FloatConverter, Float);
basic_ty_converter!(f64, DoubleConverter, Double);
basic_ty_converter!(String, StringConverter, String);
basic_ty_converter!(char, CharConverter, Character);
optional_basic_ty_converter!(i16, OptionalShortConverter, SmallInteger);
optional_basic_ty_converter!(u16, OptionalUnsignedShortConverter, UnsignedSmallInteger);
optional_basic_ty_converter!(i32, OptionalIntConverter, Integer);
optional_basic_ty_converter!(u32, OptionalUnsignedIntConverter, UnsignedInteger);
optional_basic_ty_converter!(i64, OptionalLongConverter, BigInteger);
optional_basic_ty_converter!(u64, OptionalUnsignedLongConverter, UnsignedBigInteger);
optional_basic_ty_converter!(f32, OptionalFloatConverter, Float);
optional_basic_ty_converter!(f64, OptionalDoubleConverter, Double);
optional_basic_ty_converter!(String, OptionalStringConverter, String);
optional_basic_ty_converter!(char, OptionalCharConverter, Character);
