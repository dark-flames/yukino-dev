use std::marker::PhantomData;
use std::ops::{Add, Sub};

use generic_array::typenum::Sum;

use query_builder::{Alias, Expr, GroupSelect, OrderByItem, SelectQuery};

use crate::query::{
    AliasGenerator, ExecutableSelectQuery, Filter, Filter2, Fold, Fold2, FoldQueryResult,
    FoldResult, Map, Map2, MultiRows, QueryResultMap, Sort, Sort2, SortHelper, SortResult,
};
use crate::view::{
    EmptyTagList, EntityView, EntityViewTag, EntityWithView, ExprViewBox, ExprViewBoxWithTag,
    NotInList, TagList, Value, ValueCount, ValueCountOf,
};

pub trait GroupResult: Clone {
    type Value: Value;
    type Tags: TagList;
    fn collect_expr_vec(&self) -> Vec<Expr>;

    fn expr_box(self) -> ExprViewBoxWithTag<Self::Value, Self::Tags>;
}

pub trait GroupBy<E: EntityWithView, View> {
    fn group_by<RV: GroupResult, F: Fn(View) -> RV>(self, f: F) -> GroupedQueryResult<RV, (), E>;
}

pub trait GroupFold<View: GroupResult, E: EntityWithView> {
    type Result<RV: FoldResult>;
    fn fold_group<RV: FoldResult, F: Fn(E::View) -> RV>(self, f: F) -> Self::Result<RV>;
}

pub struct GroupedQueryResult<View: GroupResult, AggregateView: Clone, E: EntityWithView> {
    query: GroupSelect,
    view: View,
    aggregate: AggregateView,
    alias_generator: AliasGenerator,
    root_alias: Alias,
    _entity: PhantomData<E>,
}

pub struct SortedGroupedQueryResult<View: GroupResult, AggregateView: Clone, E: EntityWithView> {
    nested: GroupedQueryResult<View, AggregateView, E>,
    order_by: Vec<OrderByItem>,
}

impl<View: GroupResult, E: EntityWithView> GroupedQueryResult<View, (), E> {
    pub fn create(
        query: GroupSelect,
        view: View,
        alias_generator: AliasGenerator,
        root_alias: Alias,
    ) -> Self {
        GroupedQueryResult {
            query,
            view,
            aggregate: (),
            alias_generator,
            root_alias,
            _entity: Default::default(),
        }
    }
}

impl<View: GroupResult, AggregateView: FoldResult, E: EntityWithView> Map2<View, AggregateView>
    for GroupedQueryResult<View, AggregateView, E>
{
    type ResultType = MultiRows;
    fn map<
        R: Value,
        RTags: TagList,
        RV: Into<ExprViewBoxWithTag<R, RTags>>,
        F: Fn(View, AggregateView) -> RV,
    >(
        self,
        f: F,
    ) -> QueryResultMap<R, RTags, Self::ResultType> {
        let result = f(self.view, self.aggregate).into();

        QueryResultMap::create(Box::new(self.query), vec![], result, self.alias_generator)
    }
}

impl<View: GroupResult, E: EntityWithView> Map<View> for GroupedQueryResult<View, (), E> {
    type ResultType = MultiRows;
    fn map<R: Value, RTags: TagList, RV: Into<ExprViewBoxWithTag<R, RTags>>, F: Fn(View) -> RV>(
        self,
        f: F,
    ) -> QueryResultMap<R, RTags, Self::ResultType> {
        let result = f(self.view).into();

        QueryResultMap::create(Box::new(self.query), vec![], result, self.alias_generator)
    }
}

impl<View: GroupResult, AggregateView: FoldResult, E: EntityWithView> Filter2<View, AggregateView>
    for GroupedQueryResult<View, AggregateView, E>
{
    fn filter<F, R: Into<ExprViewBox<bool>>>(mut self, f: F) -> Self
    where
        F: Fn(View, AggregateView) -> R,
    {
        let result = f(self.view.clone(), self.aggregate.clone()).into();

        self.query
            .having(result.collect_expr().into_iter().collect());

        GroupedQueryResult {
            query: self.query,
            view: self.view,
            aggregate: self.aggregate,
            alias_generator: self.alias_generator,
            root_alias: self.root_alias,
            _entity: Default::default(),
        }
    }
}

impl<View: GroupResult, E: EntityWithView> Filter<View> for GroupedQueryResult<View, (), E> {
    fn filter<F, R: Into<ExprViewBox<bool>>>(mut self, f: F) -> Self
    where
        F: Fn(View) -> R,
    {
        let result = f(self.view.clone()).into();

        self.query
            .having(result.collect_expr().into_iter().collect());

        GroupedQueryResult {
            query: self.query,
            view: self.view,
            aggregate: self.aggregate,
            alias_generator: self.alias_generator,
            root_alias: self.root_alias,
            _entity: Default::default(),
        }
    }
}

impl<View: GroupResult, AggregateView: FoldResult, E: EntityWithView> Fold2<View, AggregateView>
    for GroupedQueryResult<View, AggregateView, E>
{
    fn fold<RV: FoldResult, F: Fn(View, AggregateView) -> RV>(self, f: F) -> FoldQueryResult<RV> {
        let result = f(self.view, self.aggregate);

        FoldQueryResult::create(Box::new(self.query), result, self.alias_generator)
    }
}

impl<View: GroupResult, E: EntityWithView> Fold<View> for GroupedQueryResult<View, (), E> {
    fn fold<RV: FoldResult, F: Fn(View) -> RV>(self, f: F) -> FoldQueryResult<RV> {
        let result = f(self.view);

        FoldQueryResult::create(Box::new(self.query), result, self.alias_generator)
    }
}

impl<View: GroupResult, AggregateView: FoldResult, E: EntityWithView> Sort2<View, AggregateView>
    for GroupedQueryResult<View, AggregateView, E>
{
    type Result = SortedGroupedQueryResult<View, AggregateView, E>;

    fn sort<R: SortResult, F: Fn(View, AggregateView, SortHelper) -> R>(
        self,
        f: F,
    ) -> Self::Result {
        let result = f(
            self.view.clone(),
            self.aggregate.clone(),
            SortHelper::create(),
        );

        SortedGroupedQueryResult {
            nested: self,
            order_by: result.order_by_items(),
        }
    }
}

impl<View: GroupResult, E: EntityWithView> Sort<View> for GroupedQueryResult<View, (), E> {
    type Result = SortedGroupedQueryResult<View, (), E>;

    fn sort<R: SortResult, F: Fn(View, SortHelper) -> R>(self, f: F) -> Self::Result {
        let result = f(self.view.clone(), SortHelper::create());

        SortedGroupedQueryResult {
            nested: self,
            order_by: result.order_by_items(),
        }
    }
}

impl<View: GroupResult, E: EntityWithView> ExecutableSelectQuery<View::Value, View::Tags>
    for GroupedQueryResult<View, (), E>
{
    type ResultType = MultiRows;

    fn generate_query(self) -> (SelectQuery, ExprViewBoxWithTag<View::Value, View::Tags>) {
        (
            SelectQuery::create(
                Box::new(self.query),
                self.alias_generator
                    .generate_select_list(self.view.collect_expr_vec()),
                vec![],
                None,
                0,
            ),
            self.view.expr_box(),
        )
    }
}
type ValueTuple<V1, V2> = (<V1 as GroupResult>::Value, <V2 as FoldResult>::Value);
impl<View: GroupResult, AggregateView: FoldResult, E: EntityWithView>
    ExecutableSelectQuery<ValueTuple<View, AggregateView>, EmptyTagList>
    for GroupedQueryResult<View, AggregateView, E>
where
    ValueCountOf<View::Value>: Add<ValueCountOf<AggregateView::Value>>,
    Sum<ValueCountOf<View::Value>, ValueCountOf<AggregateView::Value>>:
        ValueCount + Sub<ValueCountOf<View::Value>, Output = ValueCountOf<AggregateView::Value>>,
{
    type ResultType = MultiRows;

    fn generate_query(
        self,
    ) -> (
        SelectQuery,
        ExprViewBoxWithTag<ValueTuple<View, AggregateView>, EmptyTagList>,
    ) {
        (
            SelectQuery::create(
                Box::new(self.query),
                self.alias_generator.generate_select_list(
                    self.view
                        .collect_expr_vec()
                        .into_iter()
                        .chain(self.aggregate.collect_fold_expr_vec()),
                ),
                vec![],
                None,
                0,
            ),
            (self.view.expr_box(), self.aggregate.expr_box()).into(),
        )
    }
}

impl<View: GroupResult, E: EntityWithView> GroupFold<View, E> for GroupedQueryResult<View, (), E> {
    type Result<RV: FoldResult> = GroupedQueryResult<View, RV, E>;

    fn fold_group<RV: FoldResult, F: Fn(E::View) -> RV>(self, f: F) -> Self::Result<RV> {
        let aggregate = f(E::View::pure(&self.root_alias));

        GroupedQueryResult {
            query: self.query,
            view: self.view,
            aggregate,
            alias_generator: self.alias_generator,
            root_alias: self.root_alias,
            _entity: Default::default(),
        }
    }
}

impl<View: GroupResult, AggregateView: FoldResult, E: EntityWithView> Map2<View, AggregateView>
    for SortedGroupedQueryResult<View, AggregateView, E>
{
    type ResultType = MultiRows;

    fn map<
        R: Value,
        RTags: TagList,
        RV: Into<ExprViewBoxWithTag<R, RTags>>,
        F: Fn(View, AggregateView) -> RV,
    >(
        self,
        f: F,
    ) -> QueryResultMap<R, RTags, Self::ResultType> {
        let result = f(self.nested.view, self.nested.aggregate).into();

        QueryResultMap::create(
            Box::new(self.nested.query),
            self.order_by,
            result,
            self.nested.alias_generator,
        )
    }
}

impl<View: GroupResult, E: EntityWithView> Map<View> for SortedGroupedQueryResult<View, (), E> {
    type ResultType = MultiRows;

    fn map<R: Value, RTags: TagList, RV: Into<ExprViewBoxWithTag<R, RTags>>, F: Fn(View) -> RV>(
        self,
        f: F,
    ) -> QueryResultMap<R, RTags, Self::ResultType> {
        let result = f(self.nested.view).into();

        QueryResultMap::create(
            Box::new(self.nested.query),
            self.order_by,
            result,
            self.nested.alias_generator,
        )
    }
}

impl<View: GroupResult, E: EntityWithView> ExecutableSelectQuery<View::Value, View::Tags>
    for SortedGroupedQueryResult<View, (), E>
{
    type ResultType = MultiRows;

    fn generate_query(self) -> (SelectQuery, ExprViewBoxWithTag<View::Value, View::Tags>) {
        (
            SelectQuery::create(
                Box::new(self.nested.query),
                self.nested
                    .alias_generator
                    .generate_select_list(self.nested.view.collect_expr_vec()),
                self.order_by,
                None,
                0,
            ),
            self.nested.view.expr_box(),
        )
    }
}

impl<View: GroupResult, AggregateView: FoldResult, E: EntityWithView>
    ExecutableSelectQuery<ValueTuple<View, AggregateView>, EmptyTagList>
    for SortedGroupedQueryResult<View, AggregateView, E>
where
    ValueCountOf<View::Value>: Add<ValueCountOf<AggregateView::Value>>,
    Sum<ValueCountOf<View::Value>, ValueCountOf<AggregateView::Value>>:
        ValueCount + Sub<ValueCountOf<View::Value>, Output = ValueCountOf<AggregateView::Value>>,
{
    type ResultType = MultiRows;

    fn generate_query(
        self,
    ) -> (
        SelectQuery,
        ExprViewBoxWithTag<ValueTuple<View, AggregateView>, EmptyTagList>,
    ) {
        (
            SelectQuery::create(
                Box::new(self.nested.query),
                self.nested.alias_generator.generate_select_list(
                    self.nested
                        .view
                        .collect_expr_vec()
                        .into_iter()
                        .chain(self.nested.aggregate.collect_fold_expr_vec()),
                ),
                vec![],
                None,
                0,
            ),
            (
                self.nested.view.expr_box(),
                self.nested.aggregate.expr_box(),
            )
                .into(),
        )
    }
}

impl<View: GroupResult, E: EntityWithView> GroupFold<View, E>
    for SortedGroupedQueryResult<View, (), E>
{
    type Result<RV: FoldResult> = SortedGroupedQueryResult<View, RV, E>;

    fn fold_group<RV: FoldResult, F: Fn(E::View) -> RV>(self, f: F) -> Self::Result<RV> {
        SortedGroupedQueryResult {
            nested: self.nested.fold_group(f),
            order_by: self.order_by,
        }
    }
}

impl<T1: Value, T1Tag: TagList> GroupResult for ExprViewBoxWithTag<T1, T1Tag>
where
    EntityViewTag: NotInList<T1Tag>,
{
    type Value = T1;
    type Tags = T1Tag;

    fn collect_expr_vec(&self) -> Vec<Expr> {
        self.collect_expr().into_iter().collect()
    }

    fn expr_box(self) -> ExprViewBoxWithTag<Self::Value, Self::Tags> {
        self
    }
}

impl<T1: Value, T1Tag: TagList, T2: Value, T2Tag: TagList> GroupResult
    for (ExprViewBoxWithTag<T1, T1Tag>, ExprViewBoxWithTag<T2, T2Tag>)
where
    EntityViewTag: NotInList<T1Tag> + NotInList<T2Tag>,
    ValueCountOf<T1>: Add<ValueCountOf<T2>>,
    Sum<ValueCountOf<T1>, ValueCountOf<T2>>:
        ValueCount + Sub<ValueCountOf<T1>, Output = ValueCountOf<T2>>,
{
    type Value = (T1, T2);
    type Tags = EmptyTagList;

    fn collect_expr_vec(&self) -> Vec<Expr> {
        self.0
            .collect_expr()
            .into_iter()
            .chain(self.1.collect_expr())
            .collect()
    }

    fn expr_box(self) -> ExprViewBoxWithTag<Self::Value, Self::Tags> {
        self.into()
    }
}
