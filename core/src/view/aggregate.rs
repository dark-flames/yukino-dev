use generic_array::functional::FunctionalSequence;
use generic_array::GenericArray;

use query_builder::{AggregateFunctionCall, DatabaseValue, Expr};

use crate::err::{RuntimeResult, YukinoError};
use crate::view::{AddTag, AggregateViewTag, ExprView, ExprViewBoxWithTag, OffsetOfTag, SetBit, TagOfValueView, True, Value, ValueCountOf};

#[derive(Clone)]
pub struct AggregatedView<T: Value> {
    pub exprs: GenericArray<Box<dyn AggregateFunctionCall>, ValueCountOf<T>>,
}

impl<T: Value> ExprView<T> for AggregatedView<T>
where
    TagOfValueView<T>: SetBit<OffsetOfTag<AggregateViewTag>, True>,
{
    type Tags = AddTag<TagOfValueView<T>, AggregateViewTag>;

    fn from_exprs(_exprs: GenericArray<Expr, ValueCountOf<T>>) -> ExprViewBoxWithTag<T, Self::Tags>
    where
        Self: Sized,
    {
        unreachable!("AggregatedView::from_exprs should never be called")
    }

    fn expr_clone(&self) -> ExprViewBoxWithTag<T, Self::Tags> {
        Box::new(self.clone())
    }

    fn clone_expr_view(&self) -> Self
    where
        Self: Sized,
    {
        self.clone()
    }

    fn collect_expr(&self) -> GenericArray<Expr, ValueCountOf<T>> {
        self.exprs
            .clone()
            .map(|expr| Expr::FunctionCall(expr.func_call_box()))
    }

    fn eval(&self, v: &GenericArray<DatabaseValue, ValueCountOf<T>>) -> RuntimeResult<T> {
        (*T::converter().deserializer())(v).map_err(|e| e.as_runtime_err())
    }
}
