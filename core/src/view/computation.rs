use crate::db::ty::DatabaseValue;
use crate::err::RuntimeResult;
use crate::view::Value;

pub trait Computation {
    type Output: Value;

    fn eval(&self, v: &[&DatabaseValue]) -> RuntimeResult<Self::Output>;
}
