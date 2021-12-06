use std::marker::PhantomData;

use query_builder::{Alias, OrderByItem, Select, SelectFrom, SelectQuery};

use crate::query::{
    AliasGenerator, ExecutableSelectQuery, Fold, FoldQueryResult, FoldResult, GroupBy, GroupedQueryResult,
    GroupResult, Map, MultiRows, QueryResultMap, Sort, SortHelper, SortResult,
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
        let view = f(E::View::pure(&self.root_alias)).into();

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
        self,
        f: F,
    ) -> QueryResultMap<R, RTags, Self::ResultType> {
        let result_view = f(E::View::pure(&self.root_alias)).into();

        QueryResultMap::create(
            Box::new(self.query),
            vec![],
            result_view,
            self.alias_generator,
        )
    }
}

impl<E: EntityWithView> Fold<E::VerticalView> for QueryResultFilter<E> {
    fn fold<RV: FoldResult, F: Fn(E::VerticalView) -> RV>(self, f: F) -> FoldQueryResult<RV> {
        let result = f(E::View::pure(&self.root_alias).vertical());

        FoldQueryResult::create(Box::new(self.query), result, self.alias_generator)
    }
}

impl<E: EntityWithView> GroupBy<E, E::View> for QueryResultFilter<E> {
    fn group_by<RV: GroupResult, F: Fn(E::View) -> RV>(
        self,
        f: F,
    ) -> GroupedQueryResult<RV, (), E> {
        let result = f(E::View::pure(&self.root_alias));

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

    fn sort<R: SortResult, F: Fn(E::View, SortHelper) -> R>(self, f: F) -> Self::Result {
        let result = f(E::View::pure(&self.root_alias), SortHelper::create());

        SortedQueryResultFilter {
            nested: self,
            order_by: result.order_by_items(),
        }
    }
}

impl<E: EntityWithView> ExecutableSelectQuery<E, TagsOfEntity<E>> for QueryResultFilter<E> {
    type ResultType = MultiRows;

    fn generate_query(self) -> (SelectQuery, ExprViewBoxWithTag<E, TagsOfEntity<E>>) {
        let view = E::View::pure(&self.root_alias);

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
    pub fn create() -> Self {
        let mut generator = AliasGenerator::create();
        let root_alias = generator.generate_root_alias::<E>();
        QueryResultFilter {
            query: Select::from(
                E::table_name().to_string(), root_alias.clone()),
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
        self,
        f: F,
    ) -> QueryResultMap<R, TTags, Self::ResultType> {
        let result_view = f(E::View::pure(&self.nested.root_alias)).into();

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

    fn generate_query(self) -> (SelectQuery, ExprViewBoxWithTag<E, TagsOfEntity<E>>) {
        let view = E::View::pure(&self.nested.root_alias);

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
