use std::marker::PhantomData;

use generic_array::{arr, GenericArray};
use generic_array::typenum::U1;

use query_builder::{DatabaseValue, Expr, ExprMutVisitor, ExprNode, ExprVisitor, FunctionCall};

use crate::err::{RuntimeResult, YukinoError};
use crate::view::{ExprView, ExprViewBox, Value, ValueCountOf, View, ViewBox};

#[derive(Clone)]
pub struct AggregateView<T: Value<L=U1>> {
    function_call: FunctionCall,
    _marker: PhantomData<T>,
}

impl<T: Value<L=U1>> ExprNode for AggregateView<T> {
    fn apply(&self, visitor: &mut dyn ExprVisitor) {
        self.function_call.apply(visitor);
    }

    fn apply_mut(&mut self, visitor: &mut dyn ExprMutVisitor) {
        self.function_call.apply_mut(visitor);
    }
}

impl<T: Value<L=U1>> View<T, ValueCountOf<T>> for AggregateView<T> {
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

impl<T: Value<L=U1>> ExprView<T> for AggregateView<T> {
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

impl<T: Value<L=U1>> AggregateView<T> {
    pub fn from_function_call(f: FunctionCall) -> Self {
        AggregateView {
            function_call: f,
            _marker: Default::default(),
        }
    }
}

pub trait ExprAverage: Value {
    type Result: Value<L=U1>;

    fn expr_average(expr: ExprViewBox<Self>) -> AggregateView<Self::Result>;
}

pub fn average<V: Value + ExprAverage>(
    v: ExprViewBox<V>,
) -> AggregateView<<V as ExprAverage>::Result> {
    V::expr_average(v)
}
