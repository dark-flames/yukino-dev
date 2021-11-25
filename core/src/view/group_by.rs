use std::ops::{Add, Sub};

use generic_array::GenericArray;
use generic_array::typenum::Sum;

use query_builder::{DatabaseValue, Expr, ExprMutVisitor, ExprNode, ExprVisitor};

use crate::err::RuntimeResult;
use crate::view::{
    ExprView, ExprViewBox, TupleExprView, Value, ValueCount, ValueCountOf, View, ViewBox,
};

pub trait GroupByView<T: Value>: ExprView<T> {}

#[derive(Clone)]
pub struct GroupByViewItem<T: Value>(ExprViewBox<T>);

impl<T: Value> ExprNode for GroupByViewItem<T> {
    fn apply(&self, visitor: &mut dyn ExprVisitor) {
        self.0.apply(visitor);
    }

    fn apply_mut(&mut self, visitor: &mut dyn ExprMutVisitor) {
        self.0.apply_mut(visitor);
    }
}

impl<T: Value> View<T, ValueCountOf<T>> for GroupByViewItem<T> {
    fn collect_expr(&self) -> GenericArray<Expr, ValueCountOf<T>> {
        self.0.collect_expr()
    }

    fn eval(&self, v: &GenericArray<DatabaseValue, ValueCountOf<T>>) -> RuntimeResult<T> {
        self.0.eval(v)
    }

    fn view_clone(&self) -> ViewBox<T, ValueCountOf<T>> {
        Box::new(self.clone())
    }
}

impl<T: Value> ExprView<T> for GroupByViewItem<T> {
    fn from_exprs(_exprs: GenericArray<Expr, ValueCountOf<T>>) -> Self
    where
        Self: Sized,
    {
        unreachable!("GroupByViewTuple cannot be construct directly");
    }

    fn expr_clone(&self) -> ExprViewBox<T> {
        Box::new(self.clone())
    }
}

impl<T: Value> GroupByView<T> for GroupByViewItem<T> {}

impl<L: Value, R: Value> GroupByView<(L, R)> for TupleExprView<L, R>
where
    ValueCountOf<L>: Add<ValueCountOf<R>>,
    Sum<ValueCountOf<L>, ValueCountOf<R>>:
        ValueCount + Sub<ValueCountOf<L>, Output = ValueCountOf<R>>,
{
}

impl<T: Value> From<ExprViewBox<T>> for GroupByViewItem<T> {
    fn from(expr: ExprViewBox<T>) -> Self {
        GroupByViewItem(expr)
    }
}
