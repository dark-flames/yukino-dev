use std::marker::PhantomData;
use std::ops::{Add, Sub};

use sqlx::Database;

use query_builder::{
    Alias, Expr, GroupSelect, IntoSelectSource, OrderByItem, SelectQuery, YukinoQuery,
};

use crate::operator::SortResult;
use crate::query::{
    AliasGenerator, Executable, Filter, Filter2, Fold, Fold2, FoldedQueryBuilder, FoldResult, Map,
    Map2, MappedQueryBuilder, MultiRows, Sort, Sort2,
};
use crate::view::{
    ConcreteList, EntityView, EntityViewTag, EntityWithView, ExprViewBoxWithTag, MergeList,
    NotInList, TagList, TagsOfValueView, Value, ValueCountOf, VerticalExprView, VerticalView,
};

pub trait GroupResult: Clone {
    type Value: Value;
    type Tags: TagList;
    type Vertical: VerticalView<Self::Value>;
    fn collect_expr_vec(&self) -> Vec<Expr>;

    fn expr_box(self) -> ExprViewBoxWithTag<Self::Value, Self::Tags>;

    fn vertical_view(self) -> Self::Vertical;
}

pub trait GroupBy<E: EntityWithView, View> {
    fn group_by<RV: GroupResult, F: Fn(View) -> RV>(self, f: F) -> GroupedQueryBuilder<RV, (), E>;
}

pub trait GroupFold<View: GroupResult, E: EntityWithView> {
    type Result<RV: FoldResult>;
    fn fold_group<RV: FoldResult, F: Fn(E::VerticalView) -> RV>(self, f: F) -> Self::Result<RV>;
}

pub struct GroupedQueryBuilder<View: GroupResult, AggregateView: Clone, E: EntityWithView> {
    query: GroupSelect,
    view: View,
    aggregate: AggregateView,
    alias_generator: AliasGenerator,
    root_alias: Alias,
    _entity: PhantomData<E>,
}

pub struct SortedGroupedQueryResult<View: GroupResult, AggregateView: Clone, E: EntityWithView> {
    nested: GroupedQueryBuilder<View, AggregateView, E>,
    order_by: Vec<OrderByItem>,
}

impl<View: GroupResult, E: EntityWithView> GroupedQueryBuilder<View, (), E> {
    pub fn create(
        query: GroupSelect,
        view: View,
        alias_generator: AliasGenerator,
        root_alias: Alias,
    ) -> Self {
        GroupedQueryBuilder {
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
    for GroupedQueryBuilder<View, AggregateView, E>
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
    ) -> MappedQueryBuilder<R, RTags, Self::ResultType> {
        let result = f(self.view, self.aggregate).into();

        MappedQueryBuilder::create(self.query.source(), vec![], result, self.alias_generator)
    }
}

impl<View: GroupResult, E: EntityWithView> Map<View> for GroupedQueryBuilder<View, (), E> {
    type ResultType = MultiRows;
    fn map<R: Value, RTags: TagList, RV: Into<ExprViewBoxWithTag<R, RTags>>, F: Fn(View) -> RV>(
        self,
        f: F,
    ) -> MappedQueryBuilder<R, RTags, Self::ResultType> {
        let result = f(self.view).into();

        MappedQueryBuilder::create(self.query.source(), vec![], result, self.alias_generator)
    }
}

impl<View: GroupResult, AggregateView: FoldResult, E: EntityWithView> Filter2<View, AggregateView>
    for GroupedQueryBuilder<View, AggregateView, E>
{
    fn filter<F, R: Into<ExprViewBoxWithTag<bool, Tags>>, Tags: TagList>(mut self, f: F) -> Self
    where
        F: Fn(View, AggregateView) -> R,
    {
        let result = f(self.view.clone(), self.aggregate.clone()).into();

        self.query
            .having(result.collect_expr().into_iter().collect());

        GroupedQueryBuilder {
            query: self.query,
            view: self.view,
            aggregate: self.aggregate,
            alias_generator: self.alias_generator,
            root_alias: self.root_alias,
            _entity: Default::default(),
        }
    }
}

impl<View: GroupResult, E: EntityWithView> Filter<View> for GroupedQueryBuilder<View, (), E> {
    fn filter<F, R: Into<ExprViewBoxWithTag<bool, Tags>>, Tags: TagList>(mut self, f: F) -> Self
    where
        F: Fn(View) -> R,
    {
        let result = f(self.view.clone()).into();

        self.query
            .having(result.collect_expr().into_iter().collect());

        GroupedQueryBuilder {
            query: self.query,
            view: self.view,
            aggregate: self.aggregate,
            alias_generator: self.alias_generator,
            root_alias: self.root_alias,
            _entity: Default::default(),
        }
    }
}

impl<View: GroupResult, AggregateView: FoldResult, E: EntityWithView>
    Fold2<View::Vertical, AggregateView> for GroupedQueryBuilder<View, AggregateView, E>
{
    fn fold<RV: FoldResult, F: Fn(View::Vertical, AggregateView) -> RV>(
        self,
        f: F,
    ) -> FoldedQueryBuilder<RV> {
        let result = f(self.view.vertical_view(), self.aggregate);

        FoldedQueryBuilder::create(self.query.source(), result, self.alias_generator)
    }
}

impl<View: GroupResult, E: EntityWithView> Fold<View::Vertical>
    for GroupedQueryBuilder<View, (), E>
{
    fn fold<RV: FoldResult, F: Fn(View::Vertical) -> RV>(self, f: F) -> FoldedQueryBuilder<RV> {
        let result = f(self.view.vertical_view());

        FoldedQueryBuilder::create(self.query.source(), result, self.alias_generator)
    }
}

impl<View: GroupResult, AggregateView: FoldResult, E: EntityWithView> Sort2<View, AggregateView>
    for GroupedQueryBuilder<View, AggregateView, E>
{
    type Result = SortedGroupedQueryResult<View, AggregateView, E>;

    fn sort<R: SortResult, F: Fn(View, AggregateView) -> R>(self, f: F) -> Self::Result {
        let result = f(self.view.clone(), self.aggregate.clone());

        SortedGroupedQueryResult {
            nested: self,
            order_by: result.order_by_items(),
        }
    }
}

impl<View: GroupResult, E: EntityWithView> Sort<View> for GroupedQueryBuilder<View, (), E> {
    type Result = SortedGroupedQueryResult<View, (), E>;

    fn sort<R: SortResult, F: Fn(View) -> R>(self, f: F) -> Self::Result {
        let result = f(self.view.clone());

        SortedGroupedQueryResult {
            nested: self,
            order_by: result.order_by_items(),
        }
    }
}

impl<View: GroupResult, E: EntityWithView, DB: Database> Executable<View::Value, DB>
    for GroupedQueryBuilder<View, (), E>
where
    SelectQuery: YukinoQuery<DB>,
{
    type ResultType = MultiRows;
    type Query = SelectQuery;

    fn generate_query(self) -> Self::Query {
        SelectQuery::create(
            self.query.source(),
            self.alias_generator
                .generate_select_list(self.view.collect_expr_vec(), true),
            vec![],
            None,
            0,
        )
    }
}

type ValueTuple<G, A> = (ValueOfGroupResult<G>, ValueOfFoldResult<A>);
type ValueOfGroupResult<G> = <G as GroupResult>::Value;
type ValueOfFoldResult<A> = <A as FoldResult>::Value;

impl<View: GroupResult, AggregateView: FoldResult, E: EntityWithView, DB: Database>
    Executable<ValueTuple<View, AggregateView>, DB> for GroupedQueryBuilder<View, AggregateView, E>
where
    SelectQuery: YukinoQuery<DB>,
    ValueTuple<View, AggregateView>: Value,
    ValueCountOf<View::Value>: Add<
        ValueCountOf<AggregateView::Value>,
        Output = ValueCountOf<ValueTuple<View, AggregateView>>,
    >,
    ValueCountOf<ValueTuple<View, AggregateView>>:
        Sub<ValueCountOf<View::Value>, Output = ValueCountOf<AggregateView::Value>>,
{
    type ResultType = MultiRows;
    type Query = SelectQuery;

    fn generate_query(self) -> Self::Query {
        SelectQuery::create(
            self.query.source(),
            self.alias_generator.generate_select_list(
                self.view
                    .collect_expr_vec()
                    .into_iter()
                    .chain(self.aggregate.collect_fold_expr_vec()),
                true,
            ),
            vec![],
            None,
            0,
        )
    }
}

impl<View: GroupResult, E: EntityWithView> GroupFold<View, E> for GroupedQueryBuilder<View, (), E> {
    type Result<RV: FoldResult> = GroupedQueryBuilder<View, RV, E>;

    fn fold_group<RV: FoldResult, F: Fn(E::VerticalView) -> RV>(self, f: F) -> Self::Result<RV> {
        let aggregate = f(E::View::pure(&self.root_alias).vertical());

        GroupedQueryBuilder {
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
    ) -> MappedQueryBuilder<R, RTags, Self::ResultType> {
        let result = f(self.nested.view, self.nested.aggregate).into();

        MappedQueryBuilder::create(
            self.nested.query.source(),
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
    ) -> MappedQueryBuilder<R, RTags, Self::ResultType> {
        let result = f(self.nested.view).into();

        MappedQueryBuilder::create(
            self.nested.query.source(),
            self.order_by,
            result,
            self.nested.alias_generator,
        )
    }
}

impl<View: GroupResult, E: EntityWithView, DB: Database> Executable<View::Value, DB>
    for SortedGroupedQueryResult<View, (), E>
where
    SelectQuery: YukinoQuery<DB>,
{
    type ResultType = MultiRows;
    type Query = SelectQuery;

    fn generate_query(self) -> Self::Query {
        SelectQuery::create(
            self.nested.query.source(),
            self.nested
                .alias_generator
                .generate_select_list(self.nested.view.collect_expr_vec(), true),
            self.order_by,
            None,
            0,
        )
    }
}

impl<View: GroupResult, AggregateView: FoldResult, E: EntityWithView, DB: Database>
    Executable<ValueTuple<View, AggregateView>, DB>
    for SortedGroupedQueryResult<View, AggregateView, E>
where
    SelectQuery: YukinoQuery<DB>,
    ValueTuple<View, AggregateView>: Value,
    ValueCountOf<View::Value>: Add<
        ValueCountOf<AggregateView::Value>,
        Output = ValueCountOf<ValueTuple<View, AggregateView>>,
    >,
    ValueCountOf<ValueTuple<View, AggregateView>>:
        Sub<ValueCountOf<View::Value>, Output = ValueCountOf<AggregateView::Value>>,
{
    type ResultType = MultiRows;
    type Query = SelectQuery;

    fn generate_query(self) -> Self::Query {
        SelectQuery::create(
            self.nested.query.source(),
            self.nested.alias_generator.generate_select_list(
                self.nested
                    .view
                    .collect_expr_vec()
                    .into_iter()
                    .chain(self.nested.aggregate.collect_fold_expr_vec()),
                true,
            ),
            vec![],
            None,
            0,
        )
    }
}

impl<View: GroupResult, E: EntityWithView> GroupFold<View, E>
    for SortedGroupedQueryResult<View, (), E>
{
    type Result<RV: FoldResult> = SortedGroupedQueryResult<View, RV, E>;

    fn fold_group<RV: FoldResult, F: Fn(E::VerticalView) -> RV>(self, f: F) -> Self::Result<RV> {
        SortedGroupedQueryResult {
            nested: self.nested.fold_group(f),
            order_by: self.order_by,
        }
    }
}

impl<T1: Value, T1Tags: TagList> GroupResult for ExprViewBoxWithTag<T1, T1Tags>
where
    EntityViewTag: NotInList<T1Tags>,
{
    type Value = T1;
    type Tags = T1Tags;
    type Vertical = VerticalExprView<T1, T1Tags>;

    fn collect_expr_vec(&self) -> Vec<Expr> {
        self.collect_expr().into_iter().collect()
    }

    fn expr_box(self) -> ExprViewBoxWithTag<Self::Value, Self::Tags> {
        self
    }

    fn vertical_view(self) -> Self::Vertical {
        VerticalExprView::create(self, vec![])
    }
}

impl<T1: Value, T1Tags: TagList + MergeList<T2Tags>, T2: Value, T2Tags: TagList> GroupResult
    for (
        ExprViewBoxWithTag<T1, T1Tags>,
        ExprViewBoxWithTag<T2, T2Tags>,
    )
where
    (T1, T2): Value,
    TagsOfValueView<T1>: MergeList<TagsOfValueView<T2>>,
    EntityViewTag: NotInList<T1Tags> + NotInList<T2Tags>,
    ValueCountOf<T1>: Add<ValueCountOf<T2>, Output = ValueCountOf<(T1, T2)>>,
    ValueCountOf<(T1, T2)>: Sub<ValueCountOf<T1>, Output = ValueCountOf<T2>>,
{
    type Value = (T1, T2);
    type Tags = ConcreteList<T1Tags, T2Tags>;
    type Vertical = (VerticalExprView<T1, T1Tags>, VerticalExprView<T2, T2Tags>);

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

    fn vertical_view(self) -> Self::Vertical {
        (self.0.vertical_view(), self.1.vertical_view())
    }
}
