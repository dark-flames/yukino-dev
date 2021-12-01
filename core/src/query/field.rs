use std::marker::PhantomData;

use query_builder::OrderByItem;

use crate::query::{QueryView, QueryViewFold, QueryViewMap, QueryViewSort};
use crate::view::{AggregateViewTag, ExprView, HasTag, SortHelper, SortResult, Value};

pub struct FieldView<T: Value, View: ExprView<T>> {
    expr_view: View,
    _ty: PhantomData<T>,
}

#[allow(dead_code)]
pub struct SortedFieldView<T: Value, View: ExprView<T>> {
    nested: FieldView<T, View>,
    order_by: Vec<OrderByItem>,
}

impl<T: Value, View: ExprView<T>> Clone for FieldView<T, View> {
    fn clone(&self) -> Self {
        FieldView {
            expr_view: self.expr_view.clone_expr_view(),
            _ty: Default::default(),
        }
    }
}

impl<T: Value, View: ExprView<T>> Clone for SortedFieldView<T, View> {
    fn clone(&self) -> Self {
        SortedFieldView {
            nested: self.nested.clone(),
            order_by: self.order_by.clone(),
        }
    }
}

impl<T: Value, View: ExprView<T>> QueryView<T> for FieldView<T, View> {
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

impl<T: Value, View: ExprView<T>> QueryView<T> for SortedFieldView<T, View> {
    type RowView = View;

    fn clone_query_view(&self) -> Self
    where
        Self: Sized,
    {
        Clone::clone(self)
    }

    fn row_view(&self) -> Self::RowView {
        self.nested.row_view()
    }
}

impl<T: Value, View: ExprView<T>> QueryViewMap<T, View> for FieldView<T, View> {
    type Output<R: Value, RV: ExprView<R>> = FieldView<R, RV>;

    fn map<R: Value, RV: ExprView<R>, IntoRV: Into<RV>, F: Fn(View) -> IntoRV>(
        self,
        f: F,
    ) -> Self::Output<R, RV>
    where
        Self: Sized,
    {
        FieldView {
            expr_view: f(self.expr_view).into(),
            _ty: Default::default(),
        }
    }
}

impl<T: Value, View: ExprView<T>> QueryViewMap<T, View> for SortedFieldView<T, View> {
    type Output<R: Value, RV: ExprView<R>> = SortedFieldView<R, RV>;

    fn map<R: Value, RV: ExprView<R>, IntoRV: Into<RV>, F: Fn(View) -> IntoRV>(
        self,
        f: F,
    ) -> Self::Output<R, RV>
    where
        Self: Sized,
    {
        SortedFieldView {
            nested: self.nested.map(f),
            order_by: self.order_by,
        }
    }
}

impl<T: Value, View: ExprView<T>> QueryViewFold<T> for FieldView<T, View> {
    type Unzipped = FieldView<T, View>;

    fn fold<R: Value, RV: ExprView<R>, IntoRV: Into<RV>, F: Fn(Self::Unzipped) -> IntoRV>(
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

impl<T: Value, View: ExprView<T>> QueryViewFold<T> for SortedFieldView<T, View> {
    type Unzipped = FieldView<T, View>;

    fn fold<R: Value, RV: ExprView<R>, IntoRV: Into<RV>, F: Fn(Self::Unzipped) -> IntoRV>(
        self,
        f: F,
    ) -> RV
    where
        Self: Sized,
        RV::Tags: HasTag<AggregateViewTag>,
    {
        f(self.nested).into()
    }
}

impl<T: Value, View: ExprView<T>> QueryViewSort<T, View> for FieldView<T, View> {
    type Output = SortedFieldView<T, View>;

    fn sort<R: SortResult, F: Fn(View, SortHelper) -> R>(self, f: F) -> Self::Output
    where
        Self: Sized,
    {
        SortedFieldView {
            nested: self.clone_query_view(),
            order_by: f(self.row_view(), SortHelper::create()).order_by_items(),
        }
    }
}

impl<T: Value, View: ExprView<T>> QueryViewSort<T, View> for SortedFieldView<T, View> {
    type Output = SortedFieldView<T, View>;

    fn sort<R: SortResult, F: Fn(View, SortHelper) -> R>(self, f: F) -> Self::Output
    where
        Self: Sized,
    {
        SortedFieldView {
            nested: self.nested.clone(),
            order_by: f(self.row_view(), SortHelper::create()).order_by_items(),
        }
    }
}

impl<T: Value, View: ExprView<T>> FieldView<T, View> {
    pub fn create(expr_view: View) -> Self {
        FieldView {
            expr_view,
            _ty: Default::default(),
        }
    }
    pub fn order_by(self, order_by: Vec<OrderByItem>) -> SortedFieldView<T, View> {
        SortedFieldView {
            nested: self,
            order_by,
        }
    }
}

impl<T: Value, View: ExprView<T>> SortedFieldView<T, View> {
    pub fn create(nested: FieldView<T, View>, order_by: Vec<OrderByItem>) -> Self {
        SortedFieldView { nested, order_by }
    }
}
