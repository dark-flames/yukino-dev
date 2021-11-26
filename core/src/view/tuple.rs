use std::ops::{Add, Sub};

use generic_array::{
    GenericArray,
    sequence::{Concat, Split},
    typenum::Sum,
};

use query_builder::{DatabaseValue, Expr, ExprMutVisitor, ExprNode, ExprVisitor};

use crate::converter::{Converter, ConverterRef, TupleConverter};
use crate::err::{RuntimeResult, YukinoError};
use crate::view::{
    EmptyTagList, ExprView, ExprViewBoxWithTag, TagList, TagOfValueView, Value, ValueCount,
    ValueCountOf, View, ViewBox,
};

pub struct TupleExprView<L: Value, R: Value, LTags: TagList, RTags: TagList>(
    pub ExprViewBoxWithTag<L, LTags>,
    pub ExprViewBoxWithTag<R, RTags>,
)
where
    ValueCountOf<L>: Add<ValueCountOf<R>>,
    Sum<ValueCountOf<L>, ValueCountOf<R>>:
        ValueCount + Sub<ValueCountOf<L>, Output = ValueCountOf<R>>;

impl<L: Value, R: Value, LTags: TagList, RTags: TagList> ExprNode
    for TupleExprView<L, R, LTags, RTags>
where
    ValueCountOf<L>: Add<ValueCountOf<R>>,
    Sum<ValueCountOf<L>, ValueCountOf<R>>:
        ValueCount + Sub<ValueCountOf<L>, Output = ValueCountOf<R>>,
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

impl<L: Value, R: Value, LTags: TagList, RTags: TagList> View<(L, R), ValueCountOf<(L, R)>>
    for TupleExprView<L, R, LTags, RTags>
where
    ValueCountOf<L>: Add<ValueCountOf<R>>,
    Sum<ValueCountOf<L>, ValueCountOf<R>>:
        ValueCount + Sub<ValueCountOf<L>, Output = ValueCountOf<R>>,
{
    fn collect_expr(&self) -> GenericArray<Expr, ValueCountOf<(L, R)>> {
        Concat::concat(self.0.collect_expr(), self.1.collect_expr())
    }

    fn eval(&self, v: &GenericArray<DatabaseValue, ValueCountOf<(L, R)>>) -> RuntimeResult<(L, R)> {
        (*<(L, R)>::converter().deserializer())(v).map_err(|e| e.as_runtime_err())
    }

    fn view_clone(&self) -> ViewBox<(L, R), ValueCountOf<(L, R)>> {
        Box::new(TupleExprView(self.0.expr_clone(), self.1.expr_clone()))
    }
}

impl<L: Value, R: Value, LTags: TagList, RTags: TagList> ExprView<(L, R)>
    for TupleExprView<L, R, LTags, RTags>
where
    ValueCountOf<L>: Add<ValueCountOf<R>>,
    Sum<ValueCountOf<L>, ValueCountOf<R>>:
        ValueCount + Sub<ValueCountOf<L>, Output = ValueCountOf<R>>,
{
    type Tags = EmptyTagList;

    fn from_exprs(
        exprs: GenericArray<Expr, ValueCountOf<(L, R)>>,
    ) -> ExprViewBoxWithTag<(L, R), Self::Tags>
    where
        Self: Sized,
    {
        let (v0, v1) = Split::split(exprs);
        Box::new(TupleExprView(
            L::view_from_exprs(v0),
            R::view_from_exprs(v1),
        ))
    }

    fn expr_clone(&self) -> ExprViewBoxWithTag<(L, R), Self::Tags> {
        Box::new(TupleExprView(self.0.expr_clone(), self.1.expr_clone()))
    }
}

impl<L: Value, R: Value> Value for (L, R)
where
    ValueCountOf<L>: Add<ValueCountOf<R>>,
    Sum<ValueCountOf<L>, ValueCountOf<R>>:
        ValueCount + Sub<ValueCountOf<L>, Output = ValueCountOf<R>>,
{
    type L = Sum<ValueCountOf<L>, ValueCountOf<R>>;
    type ValueExprView = TupleExprView<L, R, TagOfValueView<L>, TagOfValueView<R>>;

    fn converter() -> ConverterRef<Self>
    where
        Self: Sized,
    {
        TupleConverter::<L, R>::instance()
    }
}

impl<L: Value, R: Value, LTags: TagList, RTags: TagList>
    From<(ExprViewBoxWithTag<L, LTags>, ExprViewBoxWithTag<R, RTags>)>
    for TupleExprView<L, R, LTags, RTags>
where
    (L, R): Value,
    ValueCountOf<L>: Add<ValueCountOf<R>, Output = ValueCountOf<(L, R)>>,
    ValueCountOf<(L, R)>: Sub<ValueCountOf<L>, Output = ValueCountOf<R>>,
{
    fn from(tuple: (ExprViewBoxWithTag<L, LTags>, ExprViewBoxWithTag<R, RTags>)) -> Self {
        let (l, r) = tuple;
        TupleExprView(l, r)
    }
}
