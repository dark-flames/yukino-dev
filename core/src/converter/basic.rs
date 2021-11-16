use crate::converter::{ConvertResult, Converter, Deserializer};
use crate::err::ConvertError;
use crate::view::Value;
use generic_array::typenum::U1;
use generic_array::{arr, GenericArray};
use interface::DatabaseType;
use iroha::ToTokens;
use query_builder::DatabaseValue;

macro_rules! basic_ty_converter {
    ($field_type:ty, $name:ident, $enum_variant:ident, $static: ident) => {
        #[derive(ToTokens, Clone)]
        #[Iroha(mod_path = "yukino::converter::basic")]
        pub struct $name;

        static $static: $name = $name;

        impl Converter for $name {
            type Output = $field_type;

            fn instance() -> &'static Self
            where
                Self: Sized,
            {
                &$static
            }

            fn deserializer(
                &self,
            ) -> Deserializer<Self::Output> {
                Box::new(|v| {
                    if let DatabaseValue::$enum_variant(nested) = v.iter().next().unwrap() {
                        Ok(nested.clone())
                    } else {
                        Err(ConvertError::UnexpectedValueType)
                    }
                })
            }

            fn serialize(&self, value: &Self::Output) -> ConvertResult<GenericArray<DatabaseValue, <Self::Output as Value>::L>> {
                Ok(arr![DatabaseValue; DatabaseValue::$enum_variant(value.clone())])
            }
        }
    };
}

macro_rules! optional_basic_ty_converter {
    ($field_type:ty, $name:ident, $enum_variant:ident, $static: ident) => {
        #[derive(ToTokens, Clone)]
        #[Iroha(mod_path = "yukino::converter::basic")]
        pub struct $name();

        static $static: $name = $name();

        impl Converter for $name {
            type Output = Option<$field_type>;

            fn instance() -> &'static Self
            where
                Self: Sized,
            {
                &$static
            }

            fn deserializer(
                &self,
            ) -> Box<dyn Fn(&GenericArray<DatabaseValue, U1>) -> ConvertResult<Self::Output>> {
                Box::new(|v| match v.iter().next().unwrap() {
                    DatabaseValue::$enum_variant(nested) => Ok(Some(nested.clone())),
                    DatabaseValue::Null(DatabaseType::$enum_variant) => Ok(None),
                    _ => Err(ConvertError::UnexpectedValueType),
                })
            }

            fn serialize(&self, value: &Self::Output) -> ConvertResult<GenericArray<DatabaseValue, <Self::Output as Value>::L>> {
                if let Some(nested) = value {
                    Ok(arr![DatabaseValue; DatabaseValue::$enum_variant(nested.clone())])
                } else {
                    Ok(arr![DatabaseValue; DatabaseValue::Null(DatabaseType::$enum_variant)])
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
