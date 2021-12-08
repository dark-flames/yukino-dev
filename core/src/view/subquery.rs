use generic_array::{arr, GenericArray};
use generic_array::typenum::U1;

use query_builder::{DatabaseValue, Expr, SelectQuery};

use crate::err::{RuntimeResult, YukinoError};
use crate::view::{ExprView, ExprViewBox, ExprViewBoxWithTag, TagsOfValueView, Value, ValueCountOf};

// Single column query can be a subquery
pub trait SubqueryView<T: Value<L=U1>> {
    fn subquery(&self) -> SelectQuery;
}

// Single row subquery can be a SingleRowSubquery, it can be use as a ExprView
pub trait SingleRowSubqueryView<T: Value<L=U1>>: SubqueryView<T> + ExprView<T> {
}

#[derive(Clone)]
pub struct InSubqueryView {
    expr: Expr,
    subquery: SelectQuery,
}

impl ExprView<bool> for InSubqueryView {
    type Tags = TagsOfValueView<bool>;

    fn from_exprs(_exprs: GenericArray<Expr, ValueCountOf<bool>>) -> ExprViewBox<bool> where Self: Sized {
        unreachable!("InSubqueryView can not be constructed from exprs")
    }

    fn expr_clone(&self) -> ExprViewBoxWithTag<bool, Self::Tags> {
        Box::new(self.clone())
    }

    fn collect_expr(&self) -> GenericArray<Expr, ValueCountOf<bool>> {
        arr![Expr; Expr::In(Box::new(self.expr.clone()), self.subquery.clone())]
    }

    fn eval(&self, v: &GenericArray<DatabaseValue, ValueCountOf<bool>>) -> RuntimeResult<bool> {
        (*bool::converter().deserializer())(v).map_err(|e| e.as_runtime_err())
    }
}

impl InSubqueryView {
    pub fn create(expr: Expr, subquery: SelectQuery) -> Self {
        Self { expr, subquery }
    }
}