use generic_array::GenericArray;

use query_builder::{DatabaseValue, Expr};

use crate::err::RuntimeResult;
use crate::view::{TagList, TagsOfValueView, Value, ValueCountOf};

pub type ExprViewBox<T> = ExprViewBoxWithTag<T, TagsOfValueView<T>>;
pub type ExprViewBoxWithTag<T, Tags> = Box<dyn ExprView<T, Tags = Tags>>;

pub trait ExprView<T: Value> {
    type Tags: TagList;
    fn from_exprs(exprs: GenericArray<Expr, ValueCountOf<T>>) -> ExprViewBox<T>
    where
        Self: Sized;

    fn expr_clone(&self) -> ExprViewBoxWithTag<T, Self::Tags>;

    fn collect_expr(&self) -> GenericArray<Expr, ValueCountOf<T>>;

    fn eval(&self, v: &GenericArray<DatabaseValue, ValueCountOf<T>>) -> RuntimeResult<T>;
}

pub trait AnyTagExprView<T: Value>: ExprView<T> {
    fn from_exprs_with_tags(
        exprs: GenericArray<Expr, ValueCountOf<T>>,
    ) -> ExprViewBoxWithTag<T, Self::Tags>
    where
        Self: Sized;
}

impl<T: Value, TTags: TagList> Clone for ExprViewBoxWithTag<T, TTags> {
    fn clone(&self) -> Self {
        self.expr_clone()
    }
}
