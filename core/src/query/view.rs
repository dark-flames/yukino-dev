use crate::view::{AggregateViewTag, ExprView, HasTag, SortHelper, SortResult, Value};

pub trait QueryView<T: Value> {
    type RowView;

    fn clone_query_view(&self) -> Self
    where
        Self: Sized;

    fn row_view(&self) -> Self::RowView;
}

pub trait QueryViewMap<T: Value, RowView>: QueryView<T, RowView = RowView> {
    type Output<R: Value, RV: ExprView<R>>;

    fn map<R: Value, RV: ExprView<R>, IntoRV: Into<RV>, F: Fn(RowView) -> IntoRV>(
        self,
        f: F,
    ) -> Self::Output<R, RV>
    where
        Self: Sized;
}

pub trait QueryViewFold<T: Value>: QueryView<T> {
    type Unzipped;
    fn fold<R: Value, RV: ExprView<R>, IntoRV: Into<RV>, F: Fn(Self::Unzipped) -> IntoRV>(
        self,
        f: F,
    ) -> RV
    where
        Self: Sized,
        RV::Tags: HasTag<AggregateViewTag>;
}

pub trait QueryViewSort<T: Value, RowView>: QueryView<T, RowView = RowView> {
    type Output;
    fn sort<R: SortResult, F: Fn(RowView, SortHelper) -> R>(self, f: F) -> Self::Output
    where
        Self: Sized;
}

pub trait QueryViewFilter<T: Value, RowView>: QueryView<T, RowView = RowView> {
    fn filter<RV: ExprView<bool>, IntoRV: Into<RV>, F: Fn(RowView) -> IntoRV>(self, f: F) -> Self
    where
        Self: Sized;
}
