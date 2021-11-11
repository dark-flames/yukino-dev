pub mod basic;

use crate::db::ty::DatabaseValue;
use crate::err::ConvertError;
use crate::view::{Value, ValueCount};
use generic_array::GenericArray;

pub type ConverterRef<T, L> = &'static dyn Converter<L, Output = T>;

pub type ConvertResult<T> = Result<T, ConvertError>;

pub type Deserializer<T, L> = Box<dyn Fn(&GenericArray<DatabaseValue, L>) -> ConvertResult<T>>;

pub trait Converter<L: ValueCount>: Sync {
    type Output: Value<L = L>;

    fn instance() -> &'static Self
    where
        Self: Sized;

    fn deserializer(&self) -> Deserializer<Self::Output, L>;

    fn serialize(&self, value: &Self::Output) -> ConvertResult<GenericArray<DatabaseValue, L>>;
}
