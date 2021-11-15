pub mod basic;

use crate::err::ConvertError;
use crate::view::Value;
use generic_array::GenericArray;
use query_builder::DatabaseValue;

pub type ConverterRef<T> = &'static dyn Converter<Output = T>;

pub type ConvertResult<T> = Result<T, ConvertError>;

pub type Deserializer<T> =
    Box<dyn Fn(&GenericArray<DatabaseValue, <T as Value>::L>) -> ConvertResult<T>>;

pub trait Converter: Sync {
    type Output: Value;

    fn instance() -> &'static Self
    where
        Self: Sized;

    fn deserializer(&self) -> Deserializer<Self::Output>;

    fn serialize(
        &self,
        value: &Self::Output,
    ) -> ConvertResult<GenericArray<DatabaseValue, <Self::Output as Value>::L>>;
}
