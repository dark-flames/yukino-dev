use std::marker::PhantomData;

use query_builder::OrderByItem;

use crate::query::{QueryView, QueryViewFold, QueryViewMap, QueryViewSort};
use crate::query_result::{SortHelper, SortResult};
use crate::view::{AggregateViewTag, ExprView, HasTag, Value};

pub struct ColumnView<T: Value, View: ExprView<T>> {
    expr_view: View,
    _ty: PhantomData<T>,
}

#[allow(dead_code)]
pub struct SortedColumnView<T: Value, View: ExprView<T>> {
    nested: ColumnView<T, View>,
    order_by: Vec<OrderByItem>,
}

impl<'s, T: Value, View: ExprView<T>> Clone for ColumnView<T, View> {
    fn clone(&self) -> Self {
        ColumnView {
            expr_view: self.expr_view.clone_expr_view(),
            _ty: Default::default(),
        }
    }
}

impl<'s, T: Value, View: ExprView<T>> QueryView<T> for ColumnView<T, View> {
    type RowView = View;

    fn clone_query_view(&self) -> Self
    where
        Self: Sized,
    {
        Clone::clone(self)
    }

    fn row_view(&self) -> Self::RowView {
        self.expr_view.clone_expr_view()
    }
}

impl<'s, T: Value, View: ExprView<T>> QueryViewMap<T, View> for ColumnView<T, View> {
    type Output<R: Value, RV: ExprView<R>> = ColumnView<R, RV>;

    fn map<R: Value, RV: ExprView<R>, IntoRV: Into<RV>, F: Fn(View) -> IntoRV>(
        self,
        f: F,
    ) -> Self::Output<R, RV>
    where
        Self: Sized,
    {
        ColumnView {
            expr_view: f(self.expr_view).into(),
            _ty: Default::default(),
        }
    }
}

impl<'s, T: Value, View: ExprView<T>> QueryViewFold<T, View> for ColumnView<T, View> {
    type Unzip = ColumnView<T, View>;

    fn fold<R: Value, RV: ExprView<R>, IntoRV: Into<RV>, F: Fn(Self::Unzip) -> IntoRV>(
        self,
        f: F,
    ) -> RV
    where
        Self: Sized,
        RV::Tags: HasTag<AggregateViewTag>,
    {
        f(self).into()
    }
}

impl<'s, T: Value, View: ExprView<T>> QueryViewSort<T, View> for ColumnView<T, View> {
    type Output = SortedColumnView<T, View>;

    fn sort<R: SortResult, F: Fn(Self, SortHelper) -> R>(self, f: F) -> Self::Output
    where
        Self: Sized,
    {
        SortedColumnView {
            nested: self.clone_query_view(),
            order_by: f(self, SortHelper::create()).order_by_items(),
        }
    }
}
