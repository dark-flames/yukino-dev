use generic_array::GenericArray;

use query_builder::{DatabaseValue, Expr, ExprNode};

use crate::err::RuntimeResult;
use crate::view::{TagList, TagOfValueView, Value, ValueCount, ValueCountOf};

pub type ExprViewBox<T> = ExprViewBoxWithTag<T, TagOfValueView<T>>;
pub type ExprViewBoxWithTag<T, Tags> = Box<dyn ExprView<T, Tags = Tags>>;
pub type ComputationViewBox<T, L> = Box<dyn ComputationView<T, L>>;
pub type ViewBox<T, L> = Box<dyn View<T, L>>;

pub trait ExprView<T: Value>: View<T, ValueCountOf<T>> {
    type Tags: TagList;
    fn from_exprs(exprs: GenericArray<Expr, ValueCountOf<T>>) -> ExprViewBoxWithTag<T, Self::Tags>
    where
        Self: Sized;

    fn expr_clone(&self) -> ExprViewBoxWithTag<T, Self::Tags>;
}

pub trait ComputationView<T, L: ValueCount>: View<T, L> {
    fn computation_clone(&self) -> ComputationViewBox<T, L>;
}

pub trait View<T, L: ValueCount>: ExprNode {
    fn collect_expr(&self) -> GenericArray<Expr, L>;

    fn eval(&self, v: &GenericArray<DatabaseValue, L>) -> RuntimeResult<T>;

    fn view_clone(&self) -> ViewBox<T, L>;
}

impl<T: Value, TTags: TagList> Clone for ExprViewBoxWithTag<T, TTags> {
    fn clone(&self) -> Self {
        self.expr_clone()
    }
}

impl<T: 'static, L: ValueCount> Clone for ComputationViewBox<T, L> {
    fn clone(&self) -> Self {
        self.computation_clone()
    }
}

impl<T: Value, L: ValueCount> Clone for ViewBox<T, L> {
    fn clone(&self) -> Self {
        self.view_clone()
    }
}

impl<T: Value, TTags: TagList> From<ExprViewBoxWithTag<T, TTags>> for ViewBox<T, ValueCountOf<T>> {
    fn from(expr: ExprViewBoxWithTag<T, TTags>) -> Self {
        expr.view_clone()
    }
}

impl<T: 'static, L: ValueCount> From<ComputationViewBox<T, L>> for ViewBox<T, L> {
    fn from(computation: ComputationViewBox<T, L>) -> Self {
        computation.view_clone()
    }
}
