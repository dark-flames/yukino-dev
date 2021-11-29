use std::ops::{Add, Sub};

use generic_array::typenum::Sum;

use query_builder::{Expr, SelectQuery, SelectSource};

use crate::operator::AggregateHelper;
use crate::query::{AliasGenerator, ExecutableSelectQuery, ExprNode, Map, QueryResultMap};
use crate::query::exec::SingleRow;
use crate::view::{
    AggregateViewTag, EmptyTagList, ExprViewBoxWithTag, InList, TagList, Value, ValueCount,
    ValueCountOf, ViewBox,
};

pub trait FoldView: ExprNode {
    type Value: Value;
    fn collect_fold_expr_vec(&self) -> Vec<Expr>;

    fn view_box(self) -> ViewBox<Self::Value, ValueCountOf<Self::Value>>;
}

pub struct FoldQueryResult<View: FoldView> {
    query: Box<dyn SelectSource>,
    view: View,
    alias_generator: AliasGenerator,
}

impl<View: FoldView> FoldQueryResult<View> {
    pub fn create(
        query: Box<dyn SelectSource>,
        view: View,
        alias_generator: AliasGenerator,
    ) -> Self {
        FoldQueryResult {
            query,
            view,
            alias_generator,
        }
    }
}

impl<View: FoldView> Map<View> for FoldQueryResult<View> {
    type ResultType = SingleRow;
    fn map<R: 'static, RL: ValueCount, RV: Into<ViewBox<R, RL>>, F: Fn(View) -> RV>(
        mut self,
        f: F,
    ) -> QueryResultMap<R, RL, Self::ResultType> {
        let mut result = f(self.view).into();
        let mut visitor = self.alias_generator.substitute_visitor();
        result.apply_mut(&mut visitor);

        QueryResultMap::create(self.query, vec![], result)
    }
}

impl<View: FoldView> ExecutableSelectQuery<View::Value, ValueCountOf<View::Value>>
    for FoldQueryResult<View>
{
    type ResultType = SingleRow;

    fn generate_query(self) -> (SelectQuery, ViewBox<View::Value, ValueCountOf<View::Value>>) {
        (
            SelectQuery::create(
                self.query,
                self.alias_generator
                    .generate_select_list(self.view.collect_fold_expr_vec()),
                vec![],
                None,
                0,
            ),
            self.view.view_box(),
        )
    }
}

pub trait Fold<View> {
    fn fold<RV: FoldView, F: Fn(View, AggregateHelper) -> RV>(self, f: F) -> FoldQueryResult<RV>;
}

impl<T1: Value, T1Tags: TagList> FoldView for ExprViewBoxWithTag<T1, T1Tags>
where
    AggregateViewTag: InList<T1Tags>,
{
    type Value = T1;

    fn collect_fold_expr_vec(&self) -> Vec<Expr> {
        self.collect_expr().into_iter().collect()
    }

    fn view_box(self) -> ViewBox<Self::Value, ValueCountOf<Self::Value>> {
        self.into()
    }
}

impl<T1: Value, T1Tags: TagList, T2: Value, T2Tags: TagList> FoldView
    for (
        ExprViewBoxWithTag<T1, T1Tags>,
        ExprViewBoxWithTag<T2, T2Tags>,
    )
where
    AggregateViewTag: InList<T1Tags> + InList<T2Tags>,
    ValueCountOf<T1>: Add<ValueCountOf<T2>>,
    Sum<ValueCountOf<T1>, ValueCountOf<T2>>:
        ValueCount + Sub<ValueCountOf<T1>, Output = ValueCountOf<T2>>,
{
    type Value = (T1, T2);

    fn collect_fold_expr_vec(&self) -> Vec<Expr> {
        self.0
            .collect_expr()
            .into_iter()
            .chain(self.1.collect_expr())
            .collect()
    }

    fn view_box(self) -> ViewBox<Self::Value, ValueCountOf<Self::Value>> {
        let expr_view: ExprViewBoxWithTag<Self::Value, EmptyTagList> = self.into();

        expr_view.into()
    }
}
