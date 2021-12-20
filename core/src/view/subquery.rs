use std::marker::PhantomData;

use generic_array::{arr, GenericArray};
use generic_array::typenum::U1;

use query_builder::{
    DatabaseValue, Expr, FunctionCall, SelectQuery, SubqueryFunction, SubqueryFunctionCall,
};

use crate::err::{RuntimeResult, YukinoError};
use crate::view::{
    ExprView, ExprViewBox, ExprViewBoxWithTag, TagsOfValueView, Value, ValueCountOf,
};

#[derive(Clone, Copy, Debug)]
pub enum Comparator {
    Eq,
    Neq,
    Lt,
    Lte,
    Gt,
    Gte,
}

#[derive(Clone)]
pub struct InSubqueryView {
    expr: Expr,
    subquery: SelectQuery,
}

#[derive(Clone)]
pub struct ExistsView {
    subquery: SelectQuery,
}

#[derive(Clone)]
pub struct NotExistsView {
    subquery: SelectQuery,
}

pub struct SubqueryFnCallView<T: Value<L = U1>> {
    fn_call: SubqueryFunctionCall,
    _ty: PhantomData<T>,
}

// Single column query can be a subquery
pub trait SubqueryView<T: Value<L = U1>> {
    fn subquery(&self) -> SelectQuery;

    fn any(self) -> SubqueryFnCallView<T>
    where
        Self: Sized,
    {
        SubqueryFnCallView::create(self.subquery(), SubqueryFunction::Any)
    }

    fn all(self) -> SubqueryFnCallView<T>
    where
        Self: Sized,
    {
        SubqueryFnCallView::create(self.subquery(), SubqueryFunction::All)
    }
}

// Single row subquery can be a SingleRowSubquery, it can be use as a ExprView
pub trait SingleRowSubqueryView<T: Value<L = U1>>: SubqueryView<T> + ExprView<T> {}

pub(crate) trait IntoView<T: Value> {
    fn into_expr_view(self) -> ExprViewBox<T>;
}

impl ExprView<bool> for InSubqueryView {
    type Tags = TagsOfValueView<bool>;

    fn from_exprs(_exprs: GenericArray<Expr, ValueCountOf<bool>>) -> ExprViewBox<bool>
    where
        Self: Sized,
    {
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

impl ExprView<bool> for ExistsView {
    type Tags = TagsOfValueView<bool>;

    fn from_exprs(_exprs: GenericArray<Expr, ValueCountOf<bool>>) -> ExprViewBox<bool>
    where
        Self: Sized,
    {
        unreachable!("ExistsSubqueryView can not be constructed from exprs")
    }

    fn expr_clone(&self) -> ExprViewBoxWithTag<bool, Self::Tags> {
        Box::new(self.clone())
    }

    fn collect_expr(&self) -> GenericArray<Expr, ValueCountOf<bool>> {
        arr![Expr; Expr::Exists(self.subquery.clone())]
    }

    fn eval(&self, v: &GenericArray<DatabaseValue, ValueCountOf<bool>>) -> RuntimeResult<bool> {
        (*bool::converter().deserializer())(v).map_err(|e| e.as_runtime_err())
    }
}

impl ExprView<bool> for NotExistsView {
    type Tags = TagsOfValueView<bool>;

    fn from_exprs(_exprs: GenericArray<Expr, ValueCountOf<bool>>) -> ExprViewBox<bool>
    where
        Self: Sized,
    {
        unreachable!("NotExistsSubqueryView can not be constructed from exprs")
    }

    fn expr_clone(&self) -> ExprViewBoxWithTag<bool, Self::Tags> {
        Box::new(self.clone())
    }

    fn collect_expr(&self) -> GenericArray<Expr, ValueCountOf<bool>> {
        arr![Expr; Expr::NotExists(self.subquery.clone())]
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

impl ExistsView {
    pub fn create(subquery: SelectQuery) -> Self {
        Self { subquery }
    }
}

impl NotExistsView {
    pub fn create(subquery: SelectQuery) -> Self {
        Self { subquery }
    }
}

impl<T: Value<L = U1>> SubqueryFnCallView<T> {
    pub fn create(subquery: SelectQuery, func: SubqueryFunction) -> Self {
        Self {
            fn_call: SubqueryFunctionCall {
                function: func,
                subquery,
            },
            _ty: PhantomData,
        }
    }
}

impl<T: Value<L = U1>> IntoView<T> for SubqueryFnCallView<T> {
    fn into_expr_view(self) -> ExprViewBox<T> {
        T::view_from_exprs(arr![Expr; Expr::FunctionCall(self.fn_call.clone_box())])
    }
}
