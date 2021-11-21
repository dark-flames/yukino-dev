use std::marker::PhantomData;

use generic_array::typenum::U1;

use query_builder::SelectSource;

use crate::view::{AggregateView, ExprView, Value};

#[allow(dead_code)]
pub struct FoldedQueryResult<V: Value<L=U1>, View: AggregateView<V>> {
    query: Box<dyn SelectSource>,
    view: View,
    _marker: PhantomData<V>,
}

pub trait Fold<T: Value, View: ExprView<T>> {
    fn fold<R: Value<L=U1>, RV: AggregateView<R>, F: Fn(&View) -> RV>(
        self,
        f: F,
    ) -> FoldedQueryResult<R, RV>;
}

impl<V: Value<L=U1>, View: AggregateView<V>> FoldedQueryResult<V, View> {
    pub fn create(query: Box<dyn SelectSource>, view: View) -> Self {
        FoldedQueryResult {
            query,
            view,
            _marker: Default::default(),
        }
    }
}
