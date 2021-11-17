use crate::err::RuntimeResult;
use crate::view::{Value, ValueCount};
use generic_array::GenericArray;
use query_builder::{DatabaseValue, Expr};

pub type ExprViewBox<T> = Box<dyn ExprView<T>>;
pub type ComputationViewBox<T, L> = Box<dyn ComputationView<T, L>>;
pub type ViewBox<T, L> = Box<dyn View<T, L>>;

pub trait ExprView<T: Value>: View<T, <T as Value>::L> {
    fn from_exprs(exprs: GenericArray<Expr, <T as Value>::L>) -> Self
        where
            Self: Sized;

    fn expr_clone(&self) -> ExprViewBox<T>;
}

pub trait ComputationView<T, L: ValueCount>: View<T, L> {
    fn computation_clone(&self) -> ComputationViewBox<T, L>;
}

pub trait View<T, L: ValueCount> {
    fn collect_expr(&self) -> GenericArray<Expr, L>;

    fn eval(&self, v: &GenericArray<DatabaseValue, L>) -> RuntimeResult<T>;

    fn view_clone(&self) -> ViewBox<T, L>;
}

impl<T: Value> Clone for ExprViewBox<T> {
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

impl<T: Value> From<ExprViewBox<T>> for ViewBox<T, <T as Value>::L> {
    fn from(expr: ExprViewBox<T>) -> Self {
        expr.view_clone()
    }
}

impl<T: 'static, L: ValueCount> From<ComputationViewBox<T, L>> for ViewBox<T, L> {
    fn from(computation: ComputationViewBox<T, L>) -> Self {
        computation.view_clone()
    }
}
