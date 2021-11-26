use query_builder::{Expr, GroupSelect};

use crate::query::{
    AliasGenerator, ExprNode, Filter, Fold, FoldQueryResult, FoldView, Map, QueryResultMap,
};
use crate::view::{
    EntityViewTag, ExprViewBox, ExprViewBoxWithTag, NotInList, TagList, Value, ValueCount, ViewBox,
};

pub trait GroupView: Clone + ExprNode {
    fn collect_expr_vec(&self) -> Vec<Expr>;
}

pub trait GroupBy<View> {
    fn group_by<RV: GroupView, F: Fn(View) -> RV>(self, f: F) -> GroupedQueryResult<RV>;
}

pub struct GroupedQueryResult<View: GroupView> {
    query: GroupSelect,
    view: View,
    alias_generator: AliasGenerator,
}

impl<View: GroupView> GroupedQueryResult<View> {
    pub fn create(query: GroupSelect, view: View, alias_generator: AliasGenerator) -> Self {
        GroupedQueryResult {
            query,
            view,
            alias_generator,
        }
    }
}

impl<View: GroupView> Map<View> for GroupedQueryResult<View> {
    fn map<R: 'static, RL: ValueCount, RV: Into<ViewBox<R, RL>>, F: Fn(View) -> RV>(
        mut self,
        f: F,
    ) -> QueryResultMap<R, RL> {
        let mut result = f(self.view).into();
        let mut visitor = self.alias_generator.substitute_visitor();
        result.apply_mut(&mut visitor);
        QueryResultMap::create(Box::new(self.query), result, self.alias_generator)
    }
}

impl<View: GroupView> Filter<View> for GroupedQueryResult<View> {
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

impl<View: GroupView> Fold<View> for GroupedQueryResult<View> {
    fn fold<RV: FoldView, F: Fn(View) -> RV>(mut self, f: F) -> FoldQueryResult<RV> {
        let mut result = f(self.view);
        let mut visitor = self.alias_generator.substitute_visitor();
        result.apply_mut(&mut visitor);

        FoldQueryResult::create(Box::new(self.query), result, self.alias_generator)
    }
}

impl<T1: Value, T1Tag: TagList> GroupView for ExprViewBoxWithTag<T1, T1Tag>
where
    EntityViewTag: NotInList<T1Tag>,
{
    fn collect_expr_vec(&self) -> Vec<Expr> {
        self.collect_expr().into_iter().collect()
    }
}

impl<T1: Value, T1Tag: TagList, T2: Value, T2Tag: TagList> GroupView
    for (ExprViewBoxWithTag<T1, T1Tag>, ExprViewBoxWithTag<T2, T2Tag>)
where
    EntityViewTag: NotInList<T1Tag> + NotInList<T2Tag>,
{
    fn collect_expr_vec(&self) -> Vec<Expr> {
        self.0
            .collect_expr()
            .into_iter()
            .chain(self.1.collect_expr())
            .collect()
    }
}
