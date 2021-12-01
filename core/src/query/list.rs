use std::ops::{Add, Sub};

use generic_array::typenum::Sum;

use crate::query::{
    FieldView, QueryView, QueryViewFold, QueryViewMap, QueryViewSort, SortedFieldView,
};
use crate::view::{
    AggregateViewTag, ExprView, HasTag, SortHelper, SortResult, TupleExprView, Value, ValueCount,
    ValueCountOf,
};

pub trait ListView<T: Value, View: ExprView<T>>:
    QueryView<T, RowView = View>
    + QueryViewSort<T, View>
    + QueryViewFold<T, View>
    + QueryViewMap<T, View>
{
}

pub struct TupleListView<T1: Value, T2: Value, T1View: ExprView<T1>, T2View: ExprView<T2>>(
    FieldView<T1, T1View>,
    FieldView<T2, T2View>,
);

pub struct SortedTupleListView<T1: Value, T2: Value, T1View: ExprView<T1>, T2View: ExprView<T2>>(
    SortedFieldView<T1, T1View>,
    SortedFieldView<T2, T2View>,
);

impl<T1: Value, T2: Value, T1View: ExprView<T1>, T2View: ExprView<T2>> Clone
    for TupleListView<T1, T2, T1View, T2View>
{
    fn clone(&self) -> Self {
        TupleListView(self.0.clone(), self.1.clone())
    }
}

impl<T1: Value, T2: Value, T1View: ExprView<T1>, T2View: ExprView<T2>> Clone
    for SortedTupleListView<T1, T2, T1View, T2View>
{
    fn clone(&self) -> Self {
        SortedTupleListView(self.0.clone(), self.1.clone())
    }
}

impl<T1: Value, T2: Value, T1View: ExprView<T1>, T2View: ExprView<T2>> QueryView<(T1, T2)>
    for TupleListView<T1, T2, T1View, T2View>
where
    ValueCountOf<T1>: Add<ValueCountOf<T2>>,
    Sum<ValueCountOf<T1>, ValueCountOf<T2>>:
        ValueCount + Sub<ValueCountOf<T1>, Output = ValueCountOf<T2>>,
{
    type RowView = TupleExprView<T1, T2, T1View::Tags, T2View::Tags>;

    fn clone_query_view(&self) -> Self
    where
        Self: Sized,
    {
        self.clone()
    }

    fn row_view(&self) -> Self::RowView {
        TupleExprView(Box::new(self.0.row_view()), Box::new(self.1.row_view()))
    }
}

impl<T1: Value, T2: Value, T1View: ExprView<T1>, T2View: ExprView<T2>> QueryView<(T1, T2)>
    for SortedTupleListView<T1, T2, T1View, T2View>
where
    ValueCountOf<T1>: Add<ValueCountOf<T2>>,
    Sum<ValueCountOf<T1>, ValueCountOf<T2>>:
        ValueCount + Sub<ValueCountOf<T1>, Output = ValueCountOf<T2>>,
{
    type RowView = TupleExprView<T1, T2, T1View::Tags, T2View::Tags>;

    fn clone_query_view(&self) -> Self
    where
        Self: Sized,
    {
        self.clone()
    }

    fn row_view(&self) -> Self::RowView {
        TupleExprView(Box::new(self.0.row_view()), Box::new(self.1.row_view()))
    }
}

impl<T1: Value, T2: Value, T1View: ExprView<T1>, T2View: ExprView<T2>>
    QueryViewSort<(T1, T2), TupleExprView<T1, T2, T1View::Tags, T2View::Tags>>
    for TupleListView<T1, T2, T1View, T2View>
where
    ValueCountOf<T1>: Add<ValueCountOf<T2>>,
    Sum<ValueCountOf<T1>, ValueCountOf<T2>>:
        ValueCount + Sub<ValueCountOf<T1>, Output = ValueCountOf<T2>>,
{
    type Output = SortedTupleListView<T1, T2, T1View, T2View>;

    fn sort<R: SortResult, F: Fn(Self, SortHelper) -> R>(self, f: F) -> Self::Output
    where
        Self: Sized,
    {
        let result = f(self.clone(), SortHelper::create());
        SortedTupleListView(
            self.0.order_by(result.order_by_items()),
            self.1.order_by(result.order_by_items()),
        )
    }
}

impl<T1: Value, T2: Value, T1View: ExprView<T1>, T2View: ExprView<T2>>
    QueryViewFold<(T1, T2), TupleExprView<T1, T2, T1View::Tags, T2View::Tags>>
    for TupleListView<T1, T2, T1View, T2View>
where
    ValueCountOf<T1>: Add<ValueCountOf<T2>>,
    Sum<ValueCountOf<T1>, ValueCountOf<T2>>:
        ValueCount + Sub<ValueCountOf<T1>, Output = ValueCountOf<T2>>,
{
    type Unzipped = Self;

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

impl<T1: Value, T2: Value, T1View: ExprView<T1>, T2View: ExprView<T2>>
    QueryViewFold<(T1, T2), TupleExprView<T1, T2, T1View::Tags, T2View::Tags>>
    for SortedTupleListView<T1, T2, T1View, T2View>
where
    ValueCountOf<T1>: Add<ValueCountOf<T2>>,
    Sum<ValueCountOf<T1>, ValueCountOf<T2>>:
        ValueCount + Sub<ValueCountOf<T1>, Output = ValueCountOf<T2>>,
{
    type Unzipped = Self;

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

impl<T1: Value, T2: Value, T1View: ExprView<T1>, T2View: ExprView<T2>>
    QueryViewMap<(T1, T2), TupleExprView<T1, T2, T1View::Tags, T2View::Tags>>
    for TupleListView<T1, T2, T1View, T2View>
where
    ValueCountOf<T1>: Add<ValueCountOf<T2>>,
    Sum<ValueCountOf<T1>, ValueCountOf<T2>>:
        ValueCount + Sub<ValueCountOf<T1>, Output = ValueCountOf<T2>>,
{
    type Output<R: Value, RV: ExprView<R>> = FieldView<R, RV>;

    fn map<
        R: Value,
        RV: ExprView<R>,
        IntoRV: Into<RV>,
        F: Fn(TupleExprView<T1, T2, T1View::Tags, T2View::Tags>) -> IntoRV,
    >(
        self,
        f: F,
    ) -> Self::Output<R, RV>
    where
        Self: Sized,
    {
        FieldView::create(f(self.row_view()).into())
    }
}
