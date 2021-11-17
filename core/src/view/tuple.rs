use crate::converter::{Converter, ConverterRef, TupleConverter};
use crate::err::{RuntimeResult, YukinoError};
use crate::view::{ExprView, ExprViewBox, Value, ValueCount, ValueView, View, ViewBox};
use generic_array::{
    sequence::{Concat, Split},
    GenericArray,
};
use query_builder::{DatabaseValue, Expr};
use std::ops::{Add, Sub};

pub struct TupleExprView<
    V0: Value,
    V1: Value,
    OL: ValueCount + Sub<<V0 as Value>::L, Output=<V1 as Value>::L>,
> where
    <V0 as Value>::L: Add<<V1 as Value>::L, Output=OL>,
{
    view_0: ExprViewBox<V0>,
    view_1: ExprViewBox<V1>,
}

impl<V0: Value, V1: Value, OL: ValueCount + Sub<<V0 as Value>::L, Output=<V1 as Value>::L>>
View<(V0, V1), OL> for TupleExprView<V0, V1, OL>
    where
        <V0 as Value>::L: Add<<V1 as Value>::L, Output=OL>,
{
    fn eval(&self, v: &GenericArray<DatabaseValue, OL>) -> RuntimeResult<(V0, V1)> {
        (*<(V0, V1)>::converter().deserializer())(v).map_err(|e| e.as_runtime_err())
    }

    fn view_clone(&self) -> ViewBox<(V0, V1), OL> {
        Box::new(TupleExprView {
            view_0: self.view_0.expr_clone(),
            view_1: self.view_1.expr_clone(),
        })
    }
}

impl<V0: Value, V1: Value, OL: ValueCount + Sub<<V0 as Value>::L, Output=<V1 as Value>::L>>
ValueView<(V0, V1)> for TupleExprView<V0, V1, OL>
    where
        <V0 as Value>::L: Add<<V1 as Value>::L, Output=OL>,
{
    fn collect_expr(&self) -> GenericArray<Expr, <(V0, V1) as Value>::L> {
        Concat::concat(self.view_0.collect_expr(), self.view_1.collect_expr())
    }
}

impl<V0: Value, V1: Value, OL: ValueCount + Sub<<V0 as Value>::L, Output=<V1 as Value>::L>>
ExprView<(V0, V1)> for TupleExprView<V0, V1, OL>
    where
        <V0 as Value>::L: Add<<V1 as Value>::L, Output=OL>,
{
    fn from_exprs(exprs: GenericArray<Expr, <(V0, V1) as Value>::L>) -> Self
        where
            Self: Sized,
    {
        let (v0, v1) = Split::split(exprs);
        TupleExprView {
            view_0: V0::view_from_exprs(v0),
            view_1: V1::view_from_exprs(v1),
        }
    }

    fn expr_clone(&self) -> ExprViewBox<(V0, V1)> {
        Box::new(TupleExprView {
            view_0: self.view_0.expr_clone(),
            view_1: self.view_1.expr_clone(),
        })
    }
}

impl<V0: Value, V1: Value, OL: ValueCount + Sub<<V0 as Value>::L, Output=<V1 as Value>::L>> Value
for (V0, V1)
    where
        <V0 as Value>::L: Add<<V1 as Value>::L, Output=OL>,
{
    type L = OL;

    fn converter() -> ConverterRef<Self>
        where
            Self: Sized,
    {
        TupleConverter::<V0, V1>::instance()
    }

    fn view_from_exprs(exprs: GenericArray<Expr, Self::L>) -> ExprViewBox<Self>
        where
            Self: Sized,
    {
        let (view_0, view_1) = Split::split(exprs);
        Box::new(TupleExprView {
            view_0: V0::view_from_exprs(view_0),
            view_1: V1::view_from_exprs(view_1),
        })
    }
}
