use std::marker::PhantomData;

use interface::DefinitionManager;
use query_builder::{Alias, OrderByItem, Select, SelectFrom, SelectItem, SelectQuery};

use crate::operator::{AggregateHelper, AggregateHelperCreate};
use crate::query::{
    AliasGenerator, ExecutableSelectQuery, ExprNode, Fold, FoldQueryResult, FoldView, GroupBy,
    GroupedQueryResult, GroupView, Map, MultiRows, QueryResultMap, Sort, SortHelper, SortResult,
};
use crate::view::{
    EntityView, EntityWithView, ExprViewBox, ValueCount, ValueCountOf, View, ViewBox,
};

pub struct QueryResultFilter<E: EntityWithView> {
    query: SelectFrom,
    root_alias: Alias,
    alias_generator: AliasGenerator,
    _entity: PhantomData<E>,
}

pub struct SortedQueryResultFilter<E: EntityWithView> {
    nested: QueryResultFilter<E>,
    order_by: Vec<OrderByItem>,
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
        let mut view = f(E::View::pure(&self.root_alias)).into();
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
    type ResultType = MultiRows;
    fn map<R: 'static, RL: ValueCount, RV: Into<ViewBox<R, RL>>, F: Fn(E::View) -> RV>(
        mut self,
        f: F,
    ) -> QueryResultMap<R, RL, Self::ResultType> {
        let entity_view = E::View::pure(&self.root_alias);
        let mut result_view = f(entity_view).into();
        let mut visitor = self.alias_generator.substitute_visitor();
        result_view.apply_mut(&mut visitor);

        self.query.add_joins(visitor.joins());
        QueryResultMap::create(Box::new(self.query), vec![], result_view)
    }
}

impl<E: EntityWithView> Fold<E::View> for QueryResultFilter<E> {
    fn fold<RV: FoldView, F: Fn(E::View, AggregateHelper) -> RV>(
        mut self,
        f: F,
    ) -> FoldQueryResult<RV> {
        let mut visitor = self.alias_generator.substitute_visitor();
        let mut result = f(E::View::pure(&self.root_alias), AggregateHelper::create());
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

impl<E: EntityWithView> Sort<E::View> for QueryResultFilter<E> {
    type Result = SortedQueryResultFilter<E>;

    fn sort<R: SortResult, F: Fn(E::View, SortHelper) -> R>(mut self, f: F) -> Self::Result {
        let mut result = f(E::View::pure(&self.root_alias), SortHelper::create());
        let mut visitor = self.alias_generator.substitute_visitor();
        result.apply_mut(&mut visitor);

        self.query.add_joins(visitor.joins());

        SortedQueryResultFilter {
            nested: self,
            order_by: result.order_by_items(),
        }
    }
}

impl<E: EntityWithView> ExecutableSelectQuery<E, ValueCountOf<E>> for QueryResultFilter<E> {
    type ResultType = MultiRows;

    fn generate_query(mut self) -> (SelectQuery, ViewBox<E, ValueCountOf<E>>) {
        let mut view = E::View::pure(&self.root_alias);
        let mut visitor = self.alias_generator.substitute_visitor();
        view.apply_mut(&mut visitor);
        self.query.add_joins(visitor.joins());
        (
            SelectQuery::create(
                Box::new(self.query),
                view.collect_expr()
                    .into_iter()
                    .enumerate()
                    .map(|(i, e)| SelectItem {
                        expr: e,
                        alias: i.to_string(),
                    })
                    .collect(),
                vec![],
                None,
                0,
            ),
            view.view_clone(),
        )
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

impl<E: EntityWithView> Map<E::View> for SortedQueryResultFilter<E> {
    type ResultType = MultiRows;

    fn map<R: 'static, RL: ValueCount, RV: Into<ViewBox<R, RL>>, F: Fn(E::View) -> RV>(
        mut self,
        f: F,
    ) -> QueryResultMap<R, RL, Self::ResultType> {
        let entity_view = E::View::pure(&self.nested.root_alias);
        let mut result_view = f(entity_view).into();
        let mut visitor = self.nested.alias_generator.substitute_visitor();
        result_view.apply_mut(&mut visitor);

        self.nested.query.add_joins(visitor.joins());
        QueryResultMap::create(Box::new(self.nested.query), self.order_by, result_view)
    }
}

impl<E: EntityWithView> ExecutableSelectQuery<E, ValueCountOf<E>> for SortedQueryResultFilter<E> {
    type ResultType = MultiRows;

    fn generate_query(mut self) -> (SelectQuery, ViewBox<E, ValueCountOf<E>>) {
        let mut view = E::View::pure(&self.nested.root_alias);
        let mut visitor = self.nested.alias_generator.substitute_visitor();
        view.apply_mut(&mut visitor);
        self.nested.query.add_joins(visitor.joins());
        (
            SelectQuery::create(
                Box::new(self.nested.query),
                view.collect_expr()
                    .into_iter()
                    .enumerate()
                    .map(|(i, e)| SelectItem {
                        expr: e,
                        alias: i.to_string(),
                    })
                    .collect(),
                vec![],
                None,
                0,
            ),
            view.view_clone(),
        )
    }
}
