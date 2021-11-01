pub mod basic;

use crate::db::ty::DatabaseValue;
use crate::err::RuntimeResult;
use crate::expr::Value;

pub trait Converter: Sync {
    type Output: Value;

    fn deserializer(&self) -> Box<dyn Fn(&[&DatabaseValue]) -> RuntimeResult<Self::Output>>;

    fn serialize(&self, value: &Self::Output) -> RuntimeResult<Vec<DatabaseValue>>;
}
