use std::marker::PhantomData;

use query_builder::{Alias, GroupSelect};

use crate::query::{AliasGenerator, Filter, Map, QueryResultMap};
use crate::view::{EntityWithView, ExprViewBox, GroupByView, Value, ValueCount, ViewBox};

#[allow(dead_code)]
pub struct GroupedQueryResult<E: EntityWithView, V: Value, View: GroupByView<V>> {
    query: GroupSelect<E>,
    view: View,
    alias_generator: AliasGenerator,
    root_alias: Alias,
    _marker: PhantomData<V>,
}

pub trait GroupBy<E: EntityWithView> {
    fn group_by<
        R: Value,
        RView: GroupByView<R>,
        IntoRView: Into<RView>,
        F: Fn(&E::View) -> IntoRView,
    >(
        self,
        f: F,
    ) -> GroupedQueryResult<E, R, RView>;
}

impl<E: EntityWithView, V: Value, View: GroupByView<V>> GroupedQueryResult<E, V, View> {
    pub fn create(
        query: GroupSelect<E>,
        view: View,
        alias_generator: AliasGenerator,
        root_alias: Alias,
    ) -> Self {
        GroupedQueryResult {
            query,
            view,
            alias_generator,
            root_alias,
            _marker: Default::default(),
        }
    }
}

impl<E: EntityWithView, V: Value, View: GroupByView<V>> Filter<View>
    for GroupedQueryResult<E, V, View>
{
    fn filter<F, R: Into<ExprViewBox<bool>>>(&mut self, f: F)
    where
        F: Fn(&View) -> R,
    {
        let mut view = f(&self.view).into();
        let mut visitor = self.alias_generator.substitute_visitor();
        view.apply_mut(&mut visitor);

        self.query.having(view.collect_expr().into_iter().collect());
    }
}

impl<E: EntityWithView, V: Value, View: GroupByView<V>> Map<V, View>
    for GroupedQueryResult<E, V, View>
{
    fn map<R: 'static, RL: ValueCount, RV: Into<ViewBox<R, RL>>, F: Fn(&View) -> RV>(
        self,
        f: F,
    ) -> QueryResultMap<R, RL> {
        let result_view = f(&self.view).into();
        QueryResultMap::create(Box::new(self.query), self.alias_generator, result_view)
    }
}
