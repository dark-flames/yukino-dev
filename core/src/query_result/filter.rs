use std::marker::PhantomData;

use interface::DefinitionManager;
use query_builder::{Alias, OrderByItem, Select, SelectFrom, SelectQuery};

use crate::operator::{AggregateHelper, AggregateHelperCreate};
use crate::query_result::{
    AliasGenerator, ExecutableSelectQuery, ExprNode, Fold, FoldQueryResult, FoldResult, GroupBy,
    GroupedQueryResult, GroupResult, Map, MultiRows, QueryResultMap, Sort, SortHelper, SortResult,
};
use crate::view::{
    EntityView, EntityWithView, ExprView, ExprViewBox, ExprViewBoxWithTag, TagList, TagsOfEntity,
    Value,
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

pub trait Filter2<View1, View2> {
    fn filter<F, R: Into<ExprViewBox<bool>>>(self, f: F) -> Self
    where
        F: Fn(View1, View2) -> R;
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
    fn map<
        R: Value,
        RTags: TagList,
        RV: Into<ExprViewBoxWithTag<R, RTags>>,
        F: Fn(E::View) -> RV,
    >(
        mut self,
        f: F,
    ) -> QueryResultMap<R, RTags, Self::ResultType> {
        let entity_view = E::View::pure(&self.root_alias);
        let mut result_view = f(entity_view).into();
        let mut visitor = self.alias_generator.substitute_visitor();
        result_view.apply_mut(&mut visitor);

        self.query.add_joins(visitor.joins());
        QueryResultMap::create(
            Box::new(self.query),
            vec![],
            result_view,
            self.alias_generator,
        )
    }
}

impl<E: EntityWithView> Fold<E::View> for QueryResultFilter<E> {
    fn fold<RV: FoldResult, F: Fn(E::View, AggregateHelper) -> RV>(
        mut self,
        f: F,
    ) -> FoldQueryResult<RV> {
        let mut visitor = self.alias_generator.substitute_visitor();
        let mut result = f(E::View::pure(&self.root_alias), AggregateHelper::create());
        result.apply_mut(&mut visitor);

        FoldQueryResult::create(Box::new(self.query), result, self.alias_generator)
    }
}

impl<E: EntityWithView> GroupBy<E, E::View> for QueryResultFilter<E> {
    fn group_by<RV: GroupResult, F: Fn(E::View) -> RV>(
        mut self,
        f: F,
    ) -> GroupedQueryResult<RV, (), E> {
        let mut result = f(E::View::pure(&self.root_alias));
        let mut visitor = self.alias_generator.substitute_visitor();
        result.apply_mut(&mut visitor);

        GroupedQueryResult::create(
            self.query.group_by(result.collect_expr_vec()),
            result,
            self.alias_generator,
            self.root_alias,
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

impl<E: EntityWithView> ExecutableSelectQuery<E, TagsOfEntity<E>> for QueryResultFilter<E> {
    type ResultType = MultiRows;

    fn generate_query(mut self) -> (SelectQuery, ExprViewBoxWithTag<E, TagsOfEntity<E>>) {
        let mut view = E::View::pure(&self.root_alias);
        let mut visitor = self.alias_generator.substitute_visitor();
        view.apply_mut(&mut visitor);
        self.query.add_joins(visitor.joins());
        (
            SelectQuery::create(
                Box::new(self.query),
                self.alias_generator
                    .generate_select_list(view.collect_expr().into_iter()),
                vec![],
                None,
                0,
            ),
            Box::new(view),
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

    fn map<
        R: Value,
        TTags: TagList,
        RV: Into<ExprViewBoxWithTag<R, TTags>>,
        F: Fn(E::View) -> RV,
    >(
        mut self,
        f: F,
    ) -> QueryResultMap<R, TTags, Self::ResultType> {
        let entity_view = E::View::pure(&self.nested.root_alias);
        let mut result_view = f(entity_view).into();
        let mut visitor = self.nested.alias_generator.substitute_visitor();
        result_view.apply_mut(&mut visitor);

        self.nested.query.add_joins(visitor.joins());
        QueryResultMap::create(
            Box::new(self.nested.query),
            self.order_by,
            result_view,
            self.nested.alias_generator,
        )
    }
}

impl<E: EntityWithView> ExecutableSelectQuery<E, TagsOfEntity<E>> for SortedQueryResultFilter<E> {
    type ResultType = MultiRows;

    fn generate_query(mut self) -> (SelectQuery, ExprViewBoxWithTag<E, TagsOfEntity<E>>) {
        let mut view = E::View::pure(&self.nested.root_alias);
        let mut visitor = self.nested.alias_generator.substitute_visitor();
        view.apply_mut(&mut visitor);
        self.nested.query.add_joins(visitor.joins());
        (
            SelectQuery::create(
                Box::new(self.nested.query),
                self.nested
                    .alias_generator
                    .generate_select_list(view.collect_expr()),
                vec![],
                None,
                0,
            ),
            Box::new(view),
        )
    }
}
