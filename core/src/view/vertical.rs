use std::ops::{Add, Sub};

use query_builder::OrderByItem;

use crate::query::{SortHelper, SortResult};
use crate::view::{
    ExprViewBoxWithTag, MergeList, TagList, TagsOfValueView, TupleExprView, Value, ValueCountOf,
};

pub struct VerticalExprView<T: Value, TTags: TagList> {
    pub(crate) expr: ExprViewBoxWithTag<T, TTags>,
    pub(crate) order_by: Vec<OrderByItem>,
}

pub trait VerticalView<T: Value> {
    type RowView;

    fn row_view(&self) -> Self::RowView;

    fn map<
        R: Value,
        RTags: TagList,
        RV: Into<ExprViewBoxWithTag<R, RTags>>,
        F: Fn(Self::RowView) -> RV,
    >(
        self,
        f: F,
    ) -> VerticalExprView<R, RTags>;

    fn sort<R: SortResult, F: Fn(Self::RowView, SortHelper) -> R>(self, f: F) -> Self;
}

impl<T: Value, TTags: TagList> VerticalView<T> for VerticalExprView<T, TTags> {
    type RowView = ExprViewBoxWithTag<T, TTags>;

    fn row_view(&self) -> Self::RowView {
        self.expr.clone()
    }

    fn map<
        R: Value,
        RTags: TagList,
        RV: Into<ExprViewBoxWithTag<R, RTags>>,
        F: Fn(Self::RowView) -> RV,
    >(
        self,
        f: F,
    ) -> VerticalExprView<R, RTags> {
        VerticalExprView {
            expr: f(self.expr).into(),
            order_by: self.order_by,
        }
    }

    fn sort<R: SortResult, F: Fn(Self::RowView, SortHelper) -> R>(self, f: F) -> Self {
        VerticalExprView {
            expr: self.expr.clone(),
            order_by: f(self.expr, SortHelper::create()).order_by_items(),
        }
    }
}

impl<T: Value, TTags: TagList> VerticalExprView<T, TTags> {
    pub fn create(expr: ExprViewBoxWithTag<T, TTags>, order_by: Vec<OrderByItem>) -> Self {
        VerticalExprView { expr, order_by }
    }
}

impl<T1: Value, T2: Value, T1Tags: TagList, T2Tags: TagList> VerticalView<(T1, T2)>
    for (VerticalExprView<T1, T1Tags>, VerticalExprView<T2, T2Tags>)
where
    (T1, T2): Value,
    TagsOfValueView<T1>: MergeList<TagsOfValueView<T2>>,
    ValueCountOf<T1>: Add<ValueCountOf<T2>, Output = ValueCountOf<(T1, T2)>>,
    ValueCountOf<(T1, T2)>: Sub<ValueCountOf<T1>, Output = ValueCountOf<T2>>,
{
    type RowView = TupleExprView<T1, T2, T1Tags, T2Tags>;

    fn row_view(&self) -> Self::RowView {
        (self.0.row_view(), self.1.row_view()).into()
    }

    fn map<
        R: Value,
        RTags: TagList,
        RV: Into<ExprViewBoxWithTag<R, RTags>>,
        F: Fn(Self::RowView) -> RV,
    >(
        self,
        f: F,
    ) -> VerticalExprView<R, RTags> {
        VerticalExprView {
            expr: f(self.row_view()).into(),
            order_by: self.0.order_by,
        }
    }

    fn sort<R: SortResult, F: Fn(Self::RowView, SortHelper) -> R>(mut self, f: F) -> Self {
        let result = f(self.row_view(), SortHelper::create());
        let items = result.order_by_items();
        self.0.order_by = items.clone();
        self.1.order_by = items;
        (self.0, self.1)
    }
}
