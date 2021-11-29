use std::ops::{Add, Sub};

use generic_array::typenum::Sum;

use query_builder::{Expr, GroupSelect, OrderByItem, SelectQuery};

use crate::operator::{AggregateHelper, AggregateHelperCreate};
use crate::query::{
    AliasGenerator, ExecutableSelectQuery, ExprNode, Filter, Fold, FoldQueryResult, FoldResult,
    Map, MultiRows, QueryResultMap, Sort, SortHelper, SortResult,
};
use crate::view::{
    EmptyTagList, EntityViewTag, ExprViewBox, ExprViewBoxWithTag, NotInList, TagList,
    Value, ValueCount, ValueCountOf, ViewBox,
};

pub trait GroupResult: Clone + ExprNode {
    type Value: Value;
    fn collect_expr_vec(&self) -> Vec<Expr>;

    fn view_box(self) -> ViewBox<Self::Value, ValueCountOf<Self::Value>>;
}

pub trait GroupBy<View> {
    fn group_by<RV: GroupResult, F: Fn(View) -> RV>(self, f: F) -> GroupedQueryResult<RV>;
}

pub struct GroupedQueryResult<View: GroupResult> {
    query: GroupSelect,
    view: View,
    alias_generator: AliasGenerator,
}

pub struct SortedGroupedQueryResult<View: GroupResult> {
    nested: GroupedQueryResult<View>,
    order_by: Vec<OrderByItem>,
}

impl<View: GroupResult> GroupedQueryResult<View> {
    pub fn create(query: GroupSelect, view: View, alias_generator: AliasGenerator) -> Self {
        GroupedQueryResult {
            query,
            view,
            alias_generator,
        }
    }
}

impl<View: GroupResult> Map<View> for GroupedQueryResult<View> {
    type ResultType = MultiRows;
    fn map<R: 'static, RL: ValueCount, RV: Into<ViewBox<R, RL>>, F: Fn(View) -> RV>(
        mut self,
        f: F,
    ) -> QueryResultMap<R, RL, Self::ResultType> {
        let mut result = f(self.view).into();
        let mut visitor = self.alias_generator.substitute_visitor();
        result.apply_mut(&mut visitor);
        QueryResultMap::create(Box::new(self.query), vec![], result, self.alias_generator)
    }
}

impl<View: GroupResult> Filter<View> for GroupedQueryResult<View> {
    fn filter<F, R: Into<ExprViewBox<bool>>>(mut self, f: F) -> Self
    where
        F: Fn(View) -> R,
    {
        let mut result = f(self.view.clone()).into();
        let mut visitor = self.alias_generator.substitute_visitor();
        result.apply_mut(&mut visitor);
        self.query
            .having(result.collect_expr().into_iter().collect());

        GroupedQueryResult::create(self.query, self.view, self.alias_generator)
    }
}

impl<View: GroupResult> Fold<View> for GroupedQueryResult<View> {
    fn fold<RV: FoldResult, F: Fn(View, AggregateHelper) -> RV>(
        mut self,
        f: F,
    ) -> FoldQueryResult<RV> {
        let mut result = f(self.view, AggregateHelper::create());
        let mut visitor = self.alias_generator.substitute_visitor();
        result.apply_mut(&mut visitor);

        FoldQueryResult::create(Box::new(self.query), result, self.alias_generator)
    }
}

impl<View: GroupResult> Sort<View> for GroupedQueryResult<View> {
    type Result = SortedGroupedQueryResult<View>;

    fn sort<R: SortResult, F: Fn(View, SortHelper) -> R>(mut self, f: F) -> Self::Result {
        let mut result = f(self.view.clone(), SortHelper::create());
        let mut visitor = self.alias_generator.substitute_visitor();
        result.apply_mut(&mut visitor);

        SortedGroupedQueryResult {
            nested: self,
            order_by: result.order_by_items(),
        }
    }
}

impl<View: GroupResult> ExecutableSelectQuery<View::Value, ValueCountOf<View::Value>>
    for GroupedQueryResult<View>
{
    type ResultType = MultiRows;

    fn generate_query(self) -> (SelectQuery, ViewBox<View::Value, ValueCountOf<View::Value>>) {
        (
            SelectQuery::create(
                Box::new(self.query),
                self.alias_generator
                    .generate_select_list(self.view.collect_expr_vec()),
                vec![],
                None,
                0,
            ),
            self.view.view_box(),
        )
    }
}

impl<View: GroupResult> Map<View> for SortedGroupedQueryResult<View> {
    type ResultType = MultiRows;

    fn map<R: 'static, RL: ValueCount, RV: Into<ViewBox<R, RL>>, F: Fn(View) -> RV>(
        mut self,
        f: F,
    ) -> QueryResultMap<R, RL, Self::ResultType> {
        let mut result = f(self.nested.view).into();
        let mut visitor = self.nested.alias_generator.substitute_visitor();
        result.apply_mut(&mut visitor);
        QueryResultMap::create(
            Box::new(self.nested.query),
            self.order_by,
            result,
            self.nested.alias_generator,
        )
    }
}

impl<View: GroupResult> ExecutableSelectQuery<View::Value, ValueCountOf<View::Value>>
    for SortedGroupedQueryResult<View>
{
    type ResultType = MultiRows;

    fn generate_query(self) -> (SelectQuery, ViewBox<View::Value, ValueCountOf<View::Value>>) {
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
            self.nested.view.view_box(),
        )
    }
}

impl<T1: Value, T1Tag: TagList> GroupResult for ExprViewBoxWithTag<T1, T1Tag>
where
    EntityViewTag: NotInList<T1Tag>,
{
    type Value = T1;

    fn collect_expr_vec(&self) -> Vec<Expr> {
        self.collect_expr().into_iter().collect()
    }

    fn view_box(self) -> ViewBox<Self::Value, ValueCountOf<Self::Value>> {
        self.into()
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

    fn collect_expr_vec(&self) -> Vec<Expr> {
        self.0
            .collect_expr()
            .into_iter()
            .chain(self.1.collect_expr())
            .collect()
    }

    fn view_box(self) -> ViewBox<Self::Value, ValueCountOf<Self::Value>> {
        let expr: ExprViewBoxWithTag<Self::Value, EmptyTagList> = self.into();

        expr.into()
    }
}
