use crate::db::ty::ValuePack;
use crate::err::RuntimeResult;
use crate::expr::Value;

pub trait Computation {
    type Output: Value;

    fn eval(&self, v: &ValuePack) -> RuntimeResult<Self::Output>;
}
