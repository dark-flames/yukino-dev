use std::marker::PhantomData;

use interface::DefinitionManager;
use query_builder::{Alias, Select, SelectFrom};

use crate::query::{
    AliasGenerator, Fold, FoldQueryResult, FoldView, GroupBy, GroupedQueryResult, GroupView, Map,
    QueryResultMap,
};
use crate::view::{EntityView, EntityWithView, ExprViewBox, ValueCount, ViewBox};

pub struct QueryResultFilter<E: EntityWithView> {
    query: SelectFrom,
    root_alias: Alias,
    alias_generator: AliasGenerator,
    _entity: PhantomData<E>,
}

pub trait Filter<View> {
    fn filter<F, R: Into<ExprViewBox<bool>>>(self, f: F) -> Self
    where
        F: Fn(View) -> R;
}

impl<E: EntityWithView> Filter<E::View> for QueryResultFilter<E> {
    fn filter<F, R: Into<ExprViewBox<bool>>>(mut self, f: F) -> Self
    where
        F: Fn(E::View) -> R,
    {
        let entity_view = E::View::pure(&self.root_alias);
        let mut view = f(entity_view).into();
        let mut visitor = self.alias_generator.substitute_visitor();
        view.apply_mut(&mut visitor);

        self.query.add_joins(visitor.joins());
        view.collect_expr().into_iter().for_each(|e| {
            self.query.and_where(e);
        });

        self
    }
}

impl<E: EntityWithView> Map<E::View> for QueryResultFilter<E> {
    fn map<R: 'static, RL: ValueCount, RV: Into<ViewBox<R, RL>>, F: Fn(E::View) -> RV>(
        self,
        f: F,
    ) -> QueryResultMap<R, RL> {
        let entity_view = E::View::pure(&self.root_alias);
        let result_view = f(entity_view).into();

        QueryResultMap::create(Box::new(self.query), result_view, self.alias_generator)
    }
}

impl<E: EntityWithView> QueryResultFilter<E> {
    pub fn create(manager: &'static DefinitionManager) -> Self {
        let mut generator = AliasGenerator::create(manager);
        let root_alias = generator.generate_root_alias::<E>();
        QueryResultFilter {
            query: Select::from(E::definition().name.clone(), root_alias.clone()),
            root_alias,
            alias_generator: generator,
            _entity: Default::default(),
        }
    }
}

impl<E: EntityWithView> Fold<E::View> for QueryResultFilter<E> {
    fn fold<RV: FoldView, F: Fn(E::View) -> RV>(mut self, f: F) -> FoldQueryResult<RV> {
        let mut visitor = self.alias_generator.substitute_visitor();
        let mut result = f(E::View::pure(&self.root_alias));
        result.apply_mut(&mut visitor);

        FoldQueryResult::create(Box::new(self.query), result, self.alias_generator)
    }
}

impl<E: EntityWithView> GroupBy<E::View> for QueryResultFilter<E> {
    fn group_by<RV: GroupView, F: Fn(E::View) -> RV>(mut self, f: F) -> GroupedQueryResult<RV> {
        let mut result = f(E::View::pure(&self.root_alias));
        let mut visitor = self.alias_generator.substitute_visitor();
        result.apply_mut(&mut visitor);

        GroupedQueryResult::create(
            self.query.group_by(result.collect_expr_vec()),
            result,
            self.alias_generator,
        )
    }
}
