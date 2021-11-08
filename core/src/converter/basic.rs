use crate::converter::{ConvertResult, Converter};
use crate::db::ty::{DatabaseType, DatabaseValue};
use crate::err::ConvertError;
use iroha::ToTokens;

macro_rules! basic_ty_converter {
    ($field_type:ty, $name:ident, $enum_variant:ident, $static: ident) => {
        #[derive(ToTokens, Clone)]
        #[Iroha(mod_path = "yukino::converter::basic")]
        pub struct $name;

        unsafe impl Sync for $name {}

        static $static: $name = $name;

        impl Converter for $name {
            type Output = $field_type;

            fn instance() -> &'static Self
            where
                Self: Sized,
            {
                &$static
            }

            fn param_count(&self) -> usize {
                1
            }

            fn deserializer(
                &self,
            ) -> Box<dyn Fn(&[&DatabaseValue]) -> ConvertResult<Self::Output>> {
                Box::new(|v| {
                    v.first()
                        .map(|value| {
                            if let DatabaseValue::$enum_variant(nested) = value {
                                Ok(nested.clone())
                            } else {
                                Err(ConvertError::UnexpectedValueType)
                            }
                        })
                        .ok_or_else(|| ConvertError::UnexpectedValueCount)?
                })
            }

            fn serialize(&self, value: &Self::Output) -> ConvertResult<Vec<DatabaseValue>> {
                Ok(vec![DatabaseValue::$enum_variant(value.clone())])
            }
        }
    };
}

macro_rules! optional_basic_ty_converter {
    ($field_type:ty, $name:ident, $enum_variant:ident, $static: ident) => {
        #[derive(ToTokens, Clone)]
        #[Iroha(mod_path = "yukino::converter::basic")]
        pub struct $name();

        unsafe impl Sync for $name {}

        static $static: $name = $name();

        impl Converter for $name {
            type Output = Option<$field_type>;

            fn instance() -> &'static Self
            where
                Self: Sized,
            {
                &$static
            }

            fn param_count(&self) -> usize {
                1
            }

            fn deserializer(
                &self,
            ) -> Box<dyn Fn(&[&DatabaseValue]) -> ConvertResult<Self::Output>> {
                Box::new(|v| {
                    v.first()
                        .map(|value| match value {
                            DatabaseValue::$enum_variant(nested) => Ok(Some(nested.clone())),
                            DatabaseValue::Null(DatabaseType::$enum_variant) => Ok(None),
                            _ => Err(ConvertError::UnexpectedValueType),
                        })
                        .ok_or_else(|| ConvertError::UnexpectedValueCount)?
                })
            }

            fn serialize(&self, value: &Self::Output) -> ConvertResult<Vec<DatabaseValue>> {
                if let Some(nested) = value {
                    Ok(vec![DatabaseValue::$enum_variant(nested.clone())])
                } else {
                    Ok(vec![DatabaseValue::Null(DatabaseType::$enum_variant)])
                }
            }
        }
    };
}
basic_ty_converter!(bool, BoolConverter, Bool, BOOL_CONVERTER);
basic_ty_converter!(i16, ShortConverter, SmallInteger, SHORT_CONVERTER);
basic_ty_converter!(
    u16,
    UnsignedShortConverter,
    UnsignedSmallInteger,
    UNSIGNED_SHORT_CONVERTER
);
basic_ty_converter!(i32, IntConverter, Integer, INT_CONVERTER);
basic_ty_converter!(
    u32,
    UnsignedIntConverter,
    UnsignedInteger,
    UNSIGNED_CONVERTER
);
basic_ty_converter!(i64, LongConverter, BigInteger, LONG_CONVERTER);
basic_ty_converter!(
    u64,
    UnsignedLongConverter,
    UnsignedBigInteger,
    UNSIGNED_LONG_CONVERTER
);
basic_ty_converter!(f32, FloatConverter, Float, FLOAT_CONVERTER);
basic_ty_converter!(f64, DoubleConverter, Double, DOUBLE_CONVERTER);
basic_ty_converter!(String, StringConverter, String, STRING_CONVERTER);
basic_ty_converter!(char, CharConverter, Character, CHAR_CONVERTER);
optional_basic_ty_converter!(bool, OptionalBoolConverter, Bool, OPTIONAL_BOOL_CONVERTER);
optional_basic_ty_converter!(
    i16,
    OptionalShortConverter,
    SmallInteger,
    OPTIONAL_SHORT_CONVERTER
);
optional_basic_ty_converter!(
    u16,
    OptionalUnsignedShortConverter,
    UnsignedSmallInteger,
    OPTIONAL_UNSIGNED_SHORT_CONVERTER
);
optional_basic_ty_converter!(i32, OptionalIntConverter, Integer, OPTIONAL_INT_CONVERTER);
optional_basic_ty_converter!(
    u32,
    OptionalUnsignedIntConverter,
    UnsignedInteger,
    OPTIONAL_UNSIGNED_CONVERTER
);
optional_basic_ty_converter!(
    i64,
    OptionalLongConverter,
    BigInteger,
    OPTIONAL_LONG_CONVERTER
);
optional_basic_ty_converter!(
    u64,
    OptionalUnsignedLongConverter,
    UnsignedBigInteger,
    OPTIONAL_UNSIGNED_LONG_CONVERTER
);
optional_basic_ty_converter!(f32, OptionalFloatConverter, Float, OPTIONAL_FLOAT_CONVERTER);
optional_basic_ty_converter!(
    f64,
    OptionalDoubleConverter,
    Double,
    OPTIONAL_DOUBLE_CONVERTER
);
optional_basic_ty_converter!(
    String,
    OptionalStringConverter,
    String,
    OPTIONAL_STRING_CONVERTER
);
optional_basic_ty_converter!(
    char,
    OptionalCharConverter,
    Character,
    OPTIONAL_CHAR_CONVERTER
);
