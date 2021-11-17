use crate::err::RuntimeResult;
use crate::view::{
    ComputationView, ComputationViewBox, ExprViewBox, TupleExprView, Value, ValueCount, View,
    ViewBox,
};
use generic_array::{
    sequence::{Concat, Split},
    GenericArray,
};
use query_builder::{DatabaseValue, Expr, ExprMutVisitor, ExprNode, ExprVisitor};
use std::ops::{Add, Sub};

pub struct TupleComputationView<L, R, LL, RL>(ViewBox<L, LL>, ViewBox<R, RL>);

impl<
    L: 'static,
    R: 'static,
    LL: ValueCount + Add<RL, Output=OL>,
    RL: ValueCount,
    OL: ValueCount + Sub<LL, Output=RL>,
> ComputationView<(L, R), OL> for TupleComputationView<L, R, LL, RL>
{
    fn computation_clone(&self) -> ComputationViewBox<(L, R), OL> {
        Box::new(TupleComputationView(
            self.0.view_clone(),
            self.1.view_clone(),
        ))
    }
}

impl<
    L: 'static,
    R: 'static,
    LL: ValueCount + Add<RL, Output=OL>,
    RL: ValueCount,
    OL: ValueCount + Sub<LL, Output=RL>,
> ExprNode for TupleComputationView<L, R, LL, RL>
{
    fn apply(&self, visitor: &mut dyn ExprVisitor) {
        self.0.apply(visitor);
        self.1.apply(visitor);
    }

    fn apply_mut(&mut self, visitor: &mut dyn ExprMutVisitor) {
        self.0.apply_mut(visitor);
        self.1.apply_mut(visitor);
    }
}

impl<
    L: 'static,
    R: 'static,
    LL: ValueCount + Add<RL, Output=OL>,
    RL: ValueCount,
    OL: ValueCount + Sub<LL, Output=RL>,
> View<(L, R), OL> for TupleComputationView<L, R, LL, RL>
{
    fn collect_expr(&self) -> GenericArray<Expr, OL> {
        Concat::concat(self.0.collect_expr(), self.1.collect_expr())
    }

    fn eval(&self, v: &GenericArray<DatabaseValue, OL>) -> RuntimeResult<(L, R)> {
        let (input_l, input_r) = Split::split(v);
        Ok((self.0.eval(input_l)?, self.1.eval(input_r)?))
    }

    fn view_clone(&self) -> ViewBox<(L, R), OL> {
        Box::new(TupleComputationView(
            self.0.view_clone(),
            self.1.view_clone(),
        ))
    }
}

impl<L: Value, R: Value> From<(ExprViewBox<L>, ExprViewBox<R>)> for ExprViewBox<(L, R)>
    where
        (L, R): Value,
        <L as Value>::L: Add<<R as Value>::L, Output=<(L, R) as Value>::L>,
        <(L, R) as Value>::L: Sub<<L as Value>::L, Output=<R as Value>::L>,
{
    fn from(tuple: (ExprViewBox<L>, ExprViewBox<R>)) -> Self {
        let (l, r) = tuple;
        Box::new(TupleExprView(l, r))
    }
}

impl<L: Value, R: Value> From<(ExprViewBox<L>, R)> for ExprViewBox<(L, R)>
    where
        (L, R): Value,
        <L as Value>::L: Add<<R as Value>::L, Output=<(L, R) as Value>::L>,
        <(L, R) as Value>::L: Sub<<L as Value>::L, Output=<R as Value>::L>,
{
    fn from(tuple: (ExprViewBox<L>, R)) -> Self {
        let (l, r) = tuple;
        Box::new(TupleExprView(l, r.view()))
    }
}

impl<L: Value, R: 'static, RL: ValueCount, OL: ValueCount + Sub<<L as Value>::L, Output=RL>>
From<(ExprViewBox<L>, ComputationViewBox<R, RL>)> for ComputationViewBox<(L, R), OL>
    where
        <L as Value>::L: Add<RL, Output=OL>,
{
    fn from(tuple: (ExprViewBox<L>, ComputationViewBox<R, RL>)) -> Self {
        let (l, r) = tuple;
        Box::new(TupleComputationView(l.into(), r.into()))
    }
}

impl<L: Value, R: Value> From<(L, ExprViewBox<R>)> for ExprViewBox<(L, R)>
    where
        (L, R): Value,
        <L as Value>::L: Add<<R as Value>::L, Output=<(L, R) as Value>::L>,
        <(L, R) as Value>::L: Sub<<L as Value>::L, Output=<R as Value>::L>,
{
    fn from(tuple: (L, ExprViewBox<R>)) -> Self {
        let (l, r) = tuple;
        Box::new(TupleExprView(l.view(), r))
    }
}

impl<L: Value, R: 'static, RL: ValueCount, OL: ValueCount + Sub<<L as Value>::L, Output=RL>>
From<(L, ComputationViewBox<R, RL>)> for ViewBox<(L, R), OL>
    where
        <L as Value>::L: Add<RL, Output=OL>,
{
    fn from(tuple: (L, ComputationViewBox<R, RL>)) -> Self {
        let (l, r) = tuple;
        Box::new(TupleComputationView(l.view().into(), r.into()))
    }
}

impl<
    L: 'static,
    R: Value,
    LL: ValueCount + Add<<R as Value>::L, Output=OL>,
    OL: ValueCount + Sub<LL, Output=<R as Value>::L>,
> From<(ComputationViewBox<L, LL>, ExprViewBox<R>)> for ViewBox<(L, R), OL>
{
    fn from(tuple: (ComputationViewBox<L, LL>, ExprViewBox<R>)) -> Self {
        let (l, r) = tuple;
        Box::new(TupleComputationView(l.into(), r.into()))
    }
}

impl<
    L: 'static,
    R: Value,
    LL: ValueCount + Add<<R as Value>::L, Output=OL>,
    OL: ValueCount + Sub<LL, Output=<R as Value>::L>,
> From<(ComputationViewBox<L, LL>, R)> for ViewBox<(L, R), OL>
{
    fn from(tuple: (ComputationViewBox<L, LL>, R)) -> Self {
        let (l, r) = tuple;
        Box::new(TupleComputationView(l.into(), r.view().into()))
    }
}

impl<
    L: 'static,
    R: 'static,
    LL: ValueCount + Add<RL, Output=OL>,
    RL: ValueCount,
    OL: ValueCount + Sub<LL, Output=RL>,
> From<(ComputationViewBox<L, LL>, ComputationViewBox<R, RL>)> for ViewBox<(L, R), OL>
{
    fn from(tuple: (ComputationViewBox<L, LL>, ComputationViewBox<R, RL>)) -> Self {
        let (l, r) = tuple;
        Box::new(TupleComputationView(l.into(), r.into()))
    }
}

#[macro_export]
macro_rules! tuple {
    ($l: expr, $r: expr) => {
        ($l, $r).into()
    };
}
