use query_builder::SelectQuery;

use crate::query_result::{SortHelper, SortResult};
use crate::view::{AggregateViewTag, ExprView, ExprViewBoxWithTag, HasTag, Value};

pub trait QueryView<T: Value> {
    type RowView: ExprView<T>;

    fn clone_query_view(&self) -> Self
    where
        Self: Sized;

    fn row_view(&self) -> Self::RowView;
}

pub trait Executable<T: Value, Row: ExprView<T>> {
    fn generate_query(self) -> (SelectQuery, ExprViewBoxWithTag<T, Row::Tags>);
}

pub trait QueryViewMap<T: Value, Row: ExprView<T>>: QueryView<T, RowView = Row> {
    type Output<R: Value, RV: ExprView<R>>;

    fn map<R: Value, RV: ExprView<R>, IntoRV: Into<RV>, F: Fn(Row) -> IntoRV>(
        self,
        f: F,
    ) -> Self::Output<R, RV>
    where
        Self: Sized;
}

pub trait QueryViewFold<T: Value, Row: ExprView<T>>: QueryView<T, RowView = Row> {
    type Unzipped;
    fn fold<R: Value, RV: ExprView<R>, IntoRV: Into<RV>, F: Fn(Self::Unzipped) -> IntoRV>(
        self,
        f: F,
    ) -> RV
    where
        Self: Sized,
        RV::Tags: HasTag<AggregateViewTag>;
}

pub trait QueryViewSort<T: Value, Row: ExprView<T>>: QueryView<T, RowView = Row> {
    type Output;
    fn sort<R: SortResult, F: Fn(Self, SortHelper) -> R>(self, f: F) -> Self::Output
    where
        Self: Sized;
}

pub trait QueryViewFilter<T: Value, Row: ExprView<T>>: QueryView<T, RowView = Row> {
    fn filter<RV: ExprView<bool>, IntoRV: Into<RV>, F: Fn(Self) -> IntoRV>(self, f: F) -> Self
    where
        Self: Sized;
}

pub trait ListViewGroup<T: Value, Row: ExprView<T>>: QueryView<T, RowView = Row> {}
