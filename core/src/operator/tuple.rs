use std::ops::{Add, Sub};

use generic_array::{
    GenericArray,
    sequence::{Concat, Split},
};

use query_builder::{DatabaseValue, Expr};

use crate::err::RuntimeResult;
use crate::query::{ExprMutVisitor, ExprNode, ExprVisitor};
use crate::view::{
    ComputationView, ComputationViewBox, EmptyTagList, ExprViewBoxWithTag, TagList, TupleExprView,
    Value, ValueCount, ValueCountOf, View, ViewBox,
};

pub struct TupleComputationView<L, R, LL, RL>(ViewBox<L, LL>, ViewBox<R, RL>);

impl<
        L: 'static,
        R: 'static,
        LL: ValueCount + Add<RL, Output = OL>,
        RL: ValueCount,
        OL: ValueCount + Sub<LL, Output = RL>,
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
        LL: ValueCount + Add<RL, Output = OL>,
        RL: ValueCount,
        OL: ValueCount + Sub<LL, Output = RL>,
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
        LL: ValueCount + Add<RL, Output = OL>,
        RL: ValueCount,
        OL: ValueCount + Sub<LL, Output = RL>,
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

impl<L: Value, R: Value, LTags: TagList, RTags: TagList>
    From<(ExprViewBoxWithTag<L, LTags>, ExprViewBoxWithTag<R, RTags>)>
    for ExprViewBoxWithTag<(L, R), EmptyTagList>
where
    (L, R): Value,
    ValueCountOf<L>: Add<ValueCountOf<R>, Output = ValueCountOf<(L, R)>>,
    ValueCountOf<(L, R)>: Sub<ValueCountOf<L>, Output = ValueCountOf<R>>,
{
    fn from(tuple: (ExprViewBoxWithTag<L, LTags>, ExprViewBoxWithTag<R, RTags>)) -> Self {
        let (l, r) = tuple;
        Box::new(TupleExprView(l, r))
    }
}

impl<L: Value, R: Value, LTags: TagList> From<(ExprViewBoxWithTag<L, LTags>, R)>
    for ExprViewBoxWithTag<(L, R), EmptyTagList>
where
    (L, R): Value,
    ValueCountOf<L>: Add<ValueCountOf<R>, Output = ValueCountOf<(L, R)>>,
    ValueCountOf<(L, R)>: Sub<ValueCountOf<L>, Output = ValueCountOf<R>>,
{
    fn from(tuple: (ExprViewBoxWithTag<L, LTags>, R)) -> Self {
        let (l, r) = tuple;
        Box::new(TupleExprView(l, r.view()))
    }
}

impl<
        L: Value,
        R: 'static,
        RL: ValueCount,
        OL: ValueCount + Sub<ValueCountOf<L>, Output = RL>,
        LTags: TagList,
    > From<(ExprViewBoxWithTag<L, LTags>, ComputationViewBox<R, RL>)>
    for ComputationViewBox<(L, R), OL>
where
    ValueCountOf<L>: Add<RL, Output = OL>,
{
    fn from(tuple: (ExprViewBoxWithTag<L, LTags>, ComputationViewBox<R, RL>)) -> Self {
        let (l, r) = tuple;
        Box::new(TupleComputationView(l.into(), r.into()))
    }
}

impl<L: Value, R: Value, RTags: TagList> From<(L, ExprViewBoxWithTag<R, RTags>)>
    for ExprViewBoxWithTag<(L, R), EmptyTagList>
where
    (L, R): Value,
    ValueCountOf<L>: Add<ValueCountOf<R>, Output = ValueCountOf<(L, R)>>,
    ValueCountOf<(L, R)>: Sub<ValueCountOf<L>, Output = ValueCountOf<R>>,
{
    fn from(tuple: (L, ExprViewBoxWithTag<R, RTags>)) -> Self {
        let (l, r) = tuple;
        Box::new(TupleExprView(l.view(), r))
    }
}

impl<L: Value, R: 'static, RL: ValueCount, OL: ValueCount + Sub<ValueCountOf<L>, Output = RL>>
    From<(L, ComputationViewBox<R, RL>)> for ViewBox<(L, R), OL>
where
    ValueCountOf<L>: Add<RL, Output = OL>,
{
    fn from(tuple: (L, ComputationViewBox<R, RL>)) -> Self {
        let (l, r) = tuple;
        Box::new(TupleComputationView(l.view().into(), r.into()))
    }
}

impl<
        L: 'static,
        R: Value,
        LL: ValueCount + Add<ValueCountOf<R>, Output = OL>,
        OL: ValueCount + Sub<LL, Output = ValueCountOf<R>>,
        RTags: TagList,
    > From<(ComputationViewBox<L, LL>, ExprViewBoxWithTag<R, RTags>)> for ViewBox<(L, R), OL>
{
    fn from(tuple: (ComputationViewBox<L, LL>, ExprViewBoxWithTag<R, RTags>)) -> Self {
        let (l, r) = tuple;
        Box::new(TupleComputationView(l.into(), r.into()))
    }
}

impl<
        L: 'static,
        R: Value,
        LL: ValueCount + Add<ValueCountOf<R>, Output = OL>,
        OL: ValueCount + Sub<LL, Output = ValueCountOf<R>>,
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
        LL: ValueCount + Add<RL, Output = OL>,
        RL: ValueCount,
        OL: ValueCount + Sub<LL, Output = RL>,
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
