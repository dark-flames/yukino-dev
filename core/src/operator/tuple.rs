use crate::db::ty::DatabaseValue;
use crate::err::RuntimeResult;
use crate::view::{ComputationView, ComputationViewBox, ValueCount, View, ViewBox};
use generic_array::{sequence::Split, GenericArray};
use std::ops::{Add, Sub};

impl<
        L: 'static,
        R: 'static,
        LL: ValueCount + Add<RL, Output = OL>,
        RL: ValueCount,
        OL: ValueCount + Sub<LL, Output = RL>,
    > ComputationView<(L, R), OL> for (ViewBox<L, LL>, ViewBox<R, RL>)
{
    fn computation_clone(&self) -> ComputationViewBox<(L, R), OL> {
        Box::new((self.0.view_clone(), self.1.view_clone()))
    }
}

impl<
        L: 'static,
        R: 'static,
        LL: ValueCount + Add<RL, Output = OL>,
        RL: ValueCount,
        OL: ValueCount + Sub<LL, Output = RL>,
    > View<(L, R), OL> for (ViewBox<L, LL>, ViewBox<R, RL>)
{
    fn eval(&self, v: &GenericArray<DatabaseValue, OL>) -> RuntimeResult<(L, R)> {
        let (input_l, input_r) = Split::split(v);
        Ok((self.0.eval(input_l)?, self.1.eval(input_r)?))
    }

    fn view_clone(&self) -> ViewBox<(L, R), OL> {
        Box::new((self.0.view_clone(), self.1.view_clone()))
    }
}
