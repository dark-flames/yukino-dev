use crate::converter::{Converter, ConverterRef, TupleConverter};
use crate::err::{RuntimeResult, YukinoError};
use crate::view::{ExprView, ExprViewBox, Value, ValueCount, View, ViewBox};
use generic_array::{
    sequence::{Concat, Split},
    GenericArray,
};
use query_builder::{DatabaseValue, Expr};
use std::ops::{Add, Sub};

pub type TupleExprView<V0, V1> = (ExprViewBox<V0>, ExprViewBox<V1>);

impl<L: Value, R: Value, OL: ValueCount + Sub<<L as Value>::L, Output=<R as Value>::L>>
View<(L, R), <(L, R) as Value>::L> for TupleExprView<L, R>
    where
        <L as Value>::L: Add<<R as Value>::L, Output=OL>,
{
    fn collect_expr(&self) -> GenericArray<Expr, <(L, R) as Value>::L> {
        Concat::concat(self.0.collect_expr(), self.1.collect_expr())
    }

    fn eval(&self, v: &GenericArray<DatabaseValue, <(L, R) as Value>::L>) -> RuntimeResult<(L, R)> {
        (*<(L, R)>::converter().deserializer())(v).map_err(|e| e.as_runtime_err())
    }

    fn view_clone(&self) -> ViewBox<(L, R), <(L, R) as Value>::L> {
        Box::new((self.0.expr_clone(), self.1.expr_clone()))
    }
}

impl<L: Value, R: Value, OL: ValueCount + Sub<<L as Value>::L, Output=<R as Value>::L>>
ExprView<(L, R)> for TupleExprView<L, R>
    where
        <L as Value>::L: Add<<R as Value>::L, Output=OL>,
{
    fn from_exprs(exprs: GenericArray<Expr, <(L, R) as Value>::L>) -> Self
        where
            Self: Sized,
    {
        let (v0, v1) = Split::split(exprs);
        (L::view_from_exprs(v0), R::view_from_exprs(v1))
    }

    fn expr_clone(&self) -> ExprViewBox<(L, R)> {
        Box::new((self.0.expr_clone(), self.1.expr_clone()))
    }
}

impl<L: Value, R: Value, OL: ValueCount + Sub<<L as Value>::L, Output=<R as Value>::L>> Value
for (L, R)
    where
        <L as Value>::L: Add<<R as Value>::L, Output=OL>,
{
    type L = OL;

    fn converter() -> ConverterRef<Self>
        where
            Self: Sized,
    {
        TupleConverter::<L, R>::instance()
    }

    fn view_from_exprs(exprs: GenericArray<Expr, Self::L>) -> ExprViewBox<Self>
        where
            Self: Sized,
    {
        let (view_0, view_1) = Split::split(exprs);
        Box::new((L::view_from_exprs(view_0), R::view_from_exprs(view_1)))
    }
}
