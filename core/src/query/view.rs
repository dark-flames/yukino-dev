use crate::view::{AggregateViewTag, ExprView, HasTag, OrdViewTag, Value};

pub trait QueryView<T: Value> {
    type RowView: ExprView<T>;

    fn clone_query_view(&self) -> Self
    where
        Self: Sized;

    fn row_view(&self) -> Self::RowView;
}

pub trait QueryViewMap<T: Value, Row: ExprView<T>>: QueryView<T, RowView = Row> {
    type Output<RV>;

    fn make_result<R: Value, RV: ExprView<R>>(&self, r: RV) -> Self::Output<RV>;

    fn map<R: Value, RV: ExprView<R>, IntoRV: Into<RV>, F: Fn(Row) -> IntoRV>(
        self,
        f: F,
    ) -> Self::Output<RV>
    where
        Self: Sized,
    {
        self.make_result(f(self.row_view()).into())
    }
}

pub trait QueryViewFold<T: Value, Row: ExprView<T>>: QueryView<T, RowView = Row> {
    fn fold<R: Value, RV: ExprView<R>, IntoRV: Into<RV>, F: Fn(Self) -> IntoRV>(self, f: F) -> RV
    where
        Self: Sized,
        RV::Tags: HasTag<AggregateViewTag>,
    {
        f(self.clone_query_view()).into()
    }
}

pub trait QueryViewSort<T: Value, Row: ExprView<T>>: QueryView<T, RowView = Row> {
    fn sort<R: Value, RV: ExprView<R>, IntoRV: Into<RV>, F: Fn(Self) -> IntoRV>(self, f: F) -> Self
    where
        Self: Sized,
        RV::Tags: HasTag<OrdViewTag>;
}

pub trait QueryViewFilter<T: Value, Row: ExprView<T>>: QueryView<T, RowView = Row> {
    fn filter<RV: ExprView<bool>, IntoRV: Into<RV>, F: Fn(Self) -> IntoRV>(self, f: F) -> Self
    where
        Self: Sized;
}

pub trait ListViewGroup<T: Value, Row: ExprView<T>>: QueryView<T, RowView = Row> {}
