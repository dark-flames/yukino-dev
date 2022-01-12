use std::marker::PhantomData;

use generic_array::{arr, GenericArray};
use generic_array::typenum::U1;

use query_builder::{Expr, FunctionCall, SelectQuery, SubqueryFunction, SubqueryFunctionCall};

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
pub trait SingleRowSubqueryView<T: Value<L = U1>>:
    SubqueryView<T> + ExprView<T> + SubqueryIntoView<T>
{
}

pub trait SubqueryIntoView<T: Value> {
    fn as_expr(&self) -> ExprViewBox<T>;
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

impl<T: Value<L = U1>> SubqueryIntoView<T> for SubqueryFnCallView<T> {
    fn as_expr(&self) -> ExprViewBox<T> {
        T::view_from_exprs(arr![Expr; Expr::FunctionCall(
            Box::new(FunctionCall::Subquery(self.fn_call.clone()))
        )])
    }
}
