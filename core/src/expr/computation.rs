use crate::db::ty::ValuePack;
use crate::err::RuntimeResult;

pub trait Computation {
    type Output: 'static + Clone;

    fn eval(&self, v: &ValuePack) -> RuntimeResult<Self::Output>;
}
