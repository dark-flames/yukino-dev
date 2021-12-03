use std::marker::PhantomData;

use generic_array::{arr, GenericArray};
use generic_array::typenum::U1;

use query_builder::{AggregateFunctionCall, DatabaseValue, Expr};

use crate::err::{RuntimeResult, YukinoError};
use crate::view::{AggregateViewTag, ExprView, ExprViewBoxWithTag, TagList1, Value, ValueCountOf};

#[derive(Clone)]
pub struct AggregateViewItem<T: Value<L = U1>> {
    function_call: Box<dyn AggregateFunctionCall>,
    _marker: PhantomData<T>,
}

impl<T: Value<L = U1>> ExprView<T> for AggregateViewItem<T> {
    type Tags = TagList1<AggregateViewTag>;

    fn from_exprs(_exprs: GenericArray<Expr, ValueCountOf<T>>) -> ExprViewBoxWithTag<T, Self::Tags>
    where
        Self: Sized,
    {
        unreachable!("AggregateView cannot be construct directly");
    }

    fn expr_clone(&self) -> ExprViewBoxWithTag<T, Self::Tags> {
        Box::new(self.clone())
    }

    fn collect_expr(&self) -> GenericArray<Expr, ValueCountOf<T>> {
        arr![Expr; Expr::FunctionCall(self.function_call.clone_box())]
    }

    fn eval(&self, v: &GenericArray<DatabaseValue, ValueCountOf<T>>) -> RuntimeResult<T> {
        (*T::converter().deserializer())(v).map_err(|e| e.as_runtime_err())
    }
}

impl<T: Value<L = U1>> AggregateViewItem<T> {
    pub fn from_agg_fn_call<F: AggregateFunctionCall>(f: F) -> Self {
        AggregateViewItem {
            function_call: Box::new(f),
            _marker: Default::default(),
        }
    }
}
