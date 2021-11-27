use query_builder::{Expr, SelectSource};

use crate::operator::AggregateHelper;
use crate::query::{AliasGenerator, ExprNode, Map, QueryResultMap};
use crate::view::{
    AggregateViewTag, ExprViewBoxWithTag, InList, TagList, Value, ValueCount, ViewBox,
};

pub trait FoldView: ExprNode {
    fn collect_fold_expr_vec(&self) -> Vec<Expr>;
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
    fn map<R: 'static, RL: ValueCount, RV: Into<ViewBox<R, RL>>, F: Fn(View) -> RV>(
        mut self,
        f: F,
    ) -> QueryResultMap<R, RL> {
        let mut result = f(self.view).into();
        let mut visitor = self.alias_generator.substitute_visitor();
        result.apply_mut(&mut visitor);

        QueryResultMap::create(self.query, result, self.alias_generator)
    }
}

pub trait Fold<View> {
    fn fold<RV: FoldView, F: Fn(View, AggregateHelper) -> RV>(self, f: F) -> FoldQueryResult<RV>;
}

impl<T1: Value, T1Tag: TagList> FoldView for ExprViewBoxWithTag<T1, T1Tag>
where
    AggregateViewTag: InList<T1Tag>,
{
    fn collect_fold_expr_vec(&self) -> Vec<Expr> {
        self.collect_expr().into_iter().collect()
    }
}

impl<T1: Value, T1Tag: TagList, T2: Value, T2Tag: TagList> FoldView
    for (ExprViewBoxWithTag<T1, T1Tag>, ExprViewBoxWithTag<T2, T2Tag>)
where
    AggregateViewTag: InList<T1Tag> + InList<T2Tag>,
{
    fn collect_fold_expr_vec(&self) -> Vec<Expr> {
        self.0
            .collect_expr()
            .into_iter()
            .chain(self.1.collect_expr())
            .collect()
    }
}
