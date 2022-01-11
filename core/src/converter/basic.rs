use generic_array::{arr, GenericArray};
use iroha::ToTokens;
use sqlx::types::Decimal;
use sqlx::types::time::{Date, PrimitiveDateTime, Time};

use interface::DatabaseType;
use query_builder::DatabaseValue;

use crate::converter::{Converter, ConverterInstance, ConvertResult};
use crate::err::ConvertError;
use crate::view::ValueCountOf;

macro_rules! basic_ty_converter {
    ($field_type:ty, $name:ident, $enum_variant:ident) => {
        #[derive(ToTokens, Clone)]
        #[Iroha(mod_path = "yukino::converter::basic")]
        pub struct $name;

        impl Converter for $name {
            type Output = $field_type;

            fn instance() -> &'static Self
            where
                Self: Sized,
            {
                &Self::INSTANCE
            }

           fn deserialize(
                &self,
                data: GenericArray<DatabaseValue, ValueCountOf<Self::Output>>
            ) -> ConvertResult<Self::Output> {
                let [v]: [DatabaseValue; 1] = data.into();
                match v {
                    DatabaseValue::$enum_variant(nested) => Ok(nested),
                    _ => Err(ConvertError::UnexpectedValueType)
                }
            }

            fn serialize(&self, value: Self::Output) -> ConvertResult<GenericArray<DatabaseValue, ValueCountOf<Self::Output>>> {
                Ok(arr![DatabaseValue; DatabaseValue::$enum_variant(value)])
            }
        }

        impl ConverterInstance for $name {
            const INSTANCE: Self = $name;
        }
    };
}

macro_rules! optional_basic_ty_converter {
    ($field_type:ty, $name:ident, $enum_variant:ident) => {
        #[derive(ToTokens, Clone)]
        #[Iroha(mod_path = "yukino::converter::basic")]
        pub struct $name;

        impl Converter for $name {
            type Output = Option<$field_type>;

            fn instance() -> &'static Self
            where
                Self: Sized,
            {
                &Self::INSTANCE
            }

            fn deserialize(
                &self,
                data: GenericArray<DatabaseValue, ValueCountOf<Self::Output>>
            ) -> ConvertResult<Self::Output> {
                let [v]: [DatabaseValue; 1] = data.into();
                match v {
                    DatabaseValue::$enum_variant(nested) => Ok(Some(nested)),
                    DatabaseValue::Null(DatabaseType::$enum_variant) => Ok(None),
                    _ => Err(ConvertError::UnexpectedValueType)
                }
            }

            fn serialize(&self, value: Self::Output) -> ConvertResult<GenericArray<DatabaseValue, ValueCountOf<Self::Output>>> {
                if let Some(nested) = value {
                    Ok(arr![DatabaseValue; DatabaseValue::$enum_variant(nested)])
                } else {
                    Ok(arr![DatabaseValue; DatabaseValue::Null(DatabaseType::$enum_variant)])
                }
            }
        }

        impl ConverterInstance for $name {
            const INSTANCE: Self = $name;
        }
    };
}
basic_ty_converter!(bool, BoolConverter, Bool);
basic_ty_converter!(i16, ShortConverter, SmallInteger);
basic_ty_converter!(u16, UnsignedShortConverter, UnsignedSmallInteger);
basic_ty_converter!(i32, IntConverter, Integer);
basic_ty_converter!(u32, UnsignedIntConverter, UnsignedInteger);
basic_ty_converter!(i64, LongConverter, BigInteger);
basic_ty_converter!(u64, UnsignedLongConverter, UnsignedBigInteger);
basic_ty_converter!(f32, FloatConverter, Float);
basic_ty_converter!(f64, DoubleConverter, Double);
basic_ty_converter!(Decimal, DecimalConverter, Decimal);
basic_ty_converter!(Date, DateConverter, Date);
basic_ty_converter!(Time, TimeConverter, Time);
basic_ty_converter!(PrimitiveDateTime, DateTimeConverter, DateTime);
basic_ty_converter!(String, StringConverter, String);
optional_basic_ty_converter!(bool, OptionalBoolConverter, Bool);
optional_basic_ty_converter!(i16, OptionalShortConverter, SmallInteger);
optional_basic_ty_converter!(u16, OptionalUnsignedShortConverter, UnsignedSmallInteger);
optional_basic_ty_converter!(i32, OptionalIntConverter, Integer);
optional_basic_ty_converter!(u32, OptionalUnsignedIntConverter, UnsignedInteger);
optional_basic_ty_converter!(i64, OptionalLongConverter, BigInteger);
optional_basic_ty_converter!(u64, OptionalUnsignedLongConverter, UnsignedBigInteger);
optional_basic_ty_converter!(f32, OptionalFloatConverter, Float);
optional_basic_ty_converter!(f64, OptionalDoubleConverter, Double);
optional_basic_ty_converter!(Decimal, OptionalDecimalConverter, Decimal);
optional_basic_ty_converter!(Date, OptionalDateConverter, Date);
optional_basic_ty_converter!(Time, OptionalTimeConverter, Time);
optional_basic_ty_converter!(PrimitiveDateTime, OptionalDateTimeConverter, DateTime);
optional_basic_ty_converter!(String, OptionalStringConverter, String);
