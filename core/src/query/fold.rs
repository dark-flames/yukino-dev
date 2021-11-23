use std::marker::PhantomData;

use generic_array::typenum::U1;

use query_builder::SelectSource;

use crate::query::{AliasGenerator, Map, QueryResultMap};
use crate::view::{AggregateView, ExprView, Value, ValueCount, ViewBox};

#[allow(dead_code)]
pub struct FoldedQueryResult<V: Value<L=U1>, View: AggregateView<V>> {
    query: Box<dyn SelectSource>,
    view: View,
    alias_generator: AliasGenerator,
    _marker: PhantomData<V>,
}

pub trait Fold<T: Value, View: ExprView<T>> {
    fn fold<R: Value<L=U1>, RV: AggregateView<R>, F: Fn(&View) -> RV>(
        self,
        f: F,
    ) -> FoldedQueryResult<R, RV>;
}

impl<V: Value<L=U1>, View: AggregateView<V>> FoldedQueryResult<V, View> {
    pub fn create(
        query: Box<dyn SelectSource>,
        view: View,
        alias_generator: AliasGenerator,
    ) -> Self {
        FoldedQueryResult {
            query,
            view,
            alias_generator,
            _marker: Default::default(),
        }
    }
}

impl<V: Value<L=U1>, View: AggregateView<V>> Map<V, View> for FoldedQueryResult<V, View> {
    fn map<R: 'static, RL: ValueCount, RV: Into<ViewBox<R, RL>>, F: Fn(&View) -> RV>(
        self,
        f: F,
    ) -> QueryResultMap<R, RL> {
        let result_view = f(&self.view).into();

        QueryResultMap::create(self.query, self.alias_generator, result_view)
    }
}
