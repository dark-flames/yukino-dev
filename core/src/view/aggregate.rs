use std::marker::PhantomData;

use generic_array::{arr, GenericArray};
use generic_array::sequence::{Concat, Split};
use generic_array::typenum::{U1, U2};

use query_builder::{DatabaseValue, Expr, ExprMutVisitor, ExprNode, ExprVisitor, FunctionCall};

use crate::err::{RuntimeResult, YukinoError};
use crate::view::{ExprView, ExprViewBox, Value, ValueCountOf, View, ViewBox};

#[derive(Clone)]
pub struct AggregateViewItem<T: Value<L = U1>> {
    function_call: FunctionCall,
    _marker: PhantomData<T>,
}

#[derive(Clone)]
pub struct AggregateViewTuple<L: Value<L = U1>, R: Value<L = U1>>(
    pub AggregateViewItem<L>,
    pub AggregateViewItem<R>,
);

pub trait AggregateView<T: Value>: ExprView<T> {}

impl<T: Value<L = U1>> ExprNode for AggregateViewItem<T> {
    fn apply(&self, visitor: &mut dyn ExprVisitor) {
        self.function_call.apply(visitor);
    }

    fn apply_mut(&mut self, visitor: &mut dyn ExprMutVisitor) {
        self.function_call.apply_mut(visitor);
    }
}

impl<T: Value<L = U1>> View<T, ValueCountOf<T>> for AggregateViewItem<T> {
    fn collect_expr(&self) -> GenericArray<Expr, ValueCountOf<T>> {
        arr![Expr; Expr::FunctionCall(self.function_call.clone())]
    }

    fn eval(&self, v: &GenericArray<DatabaseValue, ValueCountOf<T>>) -> RuntimeResult<T> {
        (*T::converter().deserializer())(v).map_err(|e| e.as_runtime_err())
    }

    fn view_clone(&self) -> ViewBox<T, ValueCountOf<T>> {
        Box::new(self.clone())
    }
}

impl<T: Value<L = U1>> ExprView<T> for AggregateViewItem<T> {
    fn from_exprs(_exprs: GenericArray<Expr, ValueCountOf<T>>) -> Self
    where
        Self: Sized,
    {
        unreachable!("AggregateView cannot be construct directly");
    }

    fn expr_clone(&self) -> ExprViewBox<T> {
        Box::new(self.clone())
    }
}

impl<T: Value<L = U1>> AggregateViewItem<T> {
    pub fn from_function_call(f: FunctionCall) -> Self {
        AggregateViewItem {
            function_call: f,
            _marker: Default::default(),
        }
    }
}

impl<T: Value<L = U1>> AggregateView<T> for AggregateViewItem<T> {}

impl<L: Value<L = U1>, R: Value<L = U1>> ExprNode for AggregateViewTuple<L, R> {
    fn apply(&self, visitor: &mut dyn ExprVisitor) {
        self.0.apply(visitor);
        self.1.apply(visitor);
    }

    fn apply_mut(&mut self, visitor: &mut dyn ExprMutVisitor) {
        self.0.apply_mut(visitor);
        self.1.apply_mut(visitor);
    }
}

impl<L: Value<L = U1>, R: Value<L = U1>> View<(L, R), U2> for AggregateViewTuple<L, R> {
    fn collect_expr(&self) -> GenericArray<Expr, U2> {
        Concat::concat(self.0.collect_expr(), self.1.collect_expr())
    }

    fn eval(&self, v: &GenericArray<DatabaseValue, U2>) -> RuntimeResult<(L, R)> {
        let (l, r) = Split::split(v);
        Ok((self.0.eval(l)?, self.1.eval(r)?))
    }

    fn view_clone(&self) -> ViewBox<(L, R), U2> {
        Box::new(self.clone())
    }
}

impl<L: Value<L = U1>, R: Value<L = U1>> ExprView<(L, R)> for AggregateViewTuple<L, R> {
    fn from_exprs(_exprs: GenericArray<Expr, U2>) -> Self
    where
        Self: Sized,
    {
        unreachable!("AggregateView cannot be construct directly");
    }

    fn expr_clone(&self) -> ExprViewBox<(L, R)> {
        Box::new(self.clone())
    }
}

impl<L: Value<L = U1>, R: Value<L = U1>> AggregateView<(L, R)> for AggregateViewTuple<L, R> {}
