use std::marker::PhantomData;

use crate::query::QueryView;
use crate::view::{ExprView, Value};

pub struct FieldView<T: Value, View: ExprView<T>> {
    expr_view: View,
    _ty: PhantomData<T>,
}

impl<'s, T: Value, View: ExprView<T>> Clone for FieldView<T, View> {
    fn clone(&self) -> Self {
        FieldView {
            expr_view: self.expr_view.clone_expr_view(),
            _ty: Default::default(),
        }
    }
}

impl<'s, T: Value, View: ExprView<T>> QueryView<T> for FieldView<T, View> {
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
