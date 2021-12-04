use query_builder::OrderByItem;

use crate::query::{SortHelper, SortResult};
use crate::view::{ExprViewBoxWithTag, TagList, Value};

pub struct VerticalExprView<T: Value, TTags: TagList> {
    pub(crate) expr: ExprViewBoxWithTag<T, TTags>,
    pub(crate) order_by: Vec<OrderByItem>,
}

pub trait VerticalView<T: Value> {
    type RowView;

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
