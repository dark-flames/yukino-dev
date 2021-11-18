mod basic;
mod tuple;

pub use basic::*;
pub use tuple::*;

use crate::err::ConvertError;
use crate::view::{Value, ValueCountOf};
use generic_array::GenericArray;
use query_builder::DatabaseValue;

pub type ConverterRef<T> = &'static dyn Converter<Output = T>;

pub type ConvertResult<T> = Result<T, ConvertError>;

pub type Deserializer<T> =
    Box<dyn Fn(&GenericArray<DatabaseValue, ValueCountOf<T>>) -> ConvertResult<T>>;

pub trait Converter {
    type Output: Value;

    fn instance() -> &'static Self
    where
        Self: Sized;

    fn deserializer(&self) -> Deserializer<Self::Output>;

    fn serialize(
        &self,
        value: &Self::Output,
    ) -> ConvertResult<GenericArray<DatabaseValue, ValueCountOf<Self::Output>>>;
}

pub trait ConverterInstance: Converter {
    const INSTANCE: Self;
}
