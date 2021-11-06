use crate::converter::basic::*;
use crate::converter::{Converter, ConverterRef};
use std::fmt::Debug;

pub trait Value: 'static + Clone + Debug {
    fn converter() -> ConverterRef<Self>
    where
        Self: Sized;
}

pub trait CopyValue: Value + Copy {}

macro_rules! impl_value {
    ($ty: ty, $converter: ty) => {
        impl Value for $ty {
            fn converter() -> ConverterRef<Self>
            where
                Self: Sized,
            {
                <$converter>::instance()
            }
        }
    };

    ($ty: ty, $converter: ty, $optional_converter: ty) => {
        impl_value!($ty, $converter);
        impl_value!(Option<$ty>, $optional_converter);
    };
}

impl_value!(u16, UnsignedShortConverter, OptionalUnsignedShortConverter);
impl_value!(u32, UnsignedIntConverter, OptionalUnsignedIntConverter);
impl_value!(u64, UnsignedLongConverter, OptionalUnsignedLongConverter);
impl_value!(i16, ShortConverter, OptionalShortConverter);
impl_value!(i32, IntConverter, OptionalIntConverter);
impl_value!(i64, LongConverter, OptionalLongConverter);
impl_value!(f32, FloatConverter, OptionalFloatConverter);
impl_value!(f64, DoubleConverter, OptionalDoubleConverter);
impl_value!(char, CharConverter, OptionalCharConverter);
impl_value!(String, StringConverter, OptionalStringConverter);
