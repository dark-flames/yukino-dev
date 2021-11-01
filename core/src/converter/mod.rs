pub mod basic;

use crate::db::ty::DatabaseValue;
use crate::err::RuntimeResult;

pub trait Converter: Sync {
    type Output: 'static + Clone;

    fn deserializer(&self) -> Box<dyn Fn(&[&DatabaseValue]) -> RuntimeResult<Self::Output>>;

    fn serialize(&self, value: &Self::Output) -> RuntimeResult<Vec<DatabaseValue>>;
}
