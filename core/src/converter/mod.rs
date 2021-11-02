pub mod basic;

use crate::db::ty::DatabaseValue;
use crate::err::RuntimeResult;
use crate::view::Value;

pub type ConverterRef<T> = &'static dyn Converter<Output=T>;

pub trait Converter: Sync {
    type Output: Value;

    fn instance() -> &'static Self
        where
            Self: Sized;

    fn param_count(&self) -> usize;

    fn deserializer(&self) -> Box<dyn Fn(&[&DatabaseValue]) -> RuntimeResult<Self::Output>>;

    fn serialize(&self, value: &Self::Output) -> RuntimeResult<Vec<DatabaseValue>>;
}
