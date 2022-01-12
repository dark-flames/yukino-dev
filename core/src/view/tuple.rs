use std::ops::{Add, Sub};

use generic_array::{ArrayLength, GenericArray, sequence::Concat, typenum::Sum};
use generic_array::sequence::Split;
use sqlx::Database;

use query_builder::{ColumnOf, DatabaseValue, Expr, RowOf};

use crate::view::{
    ConcreteList, EvalResult, ExprView, ExprViewBox, ExprViewBoxWithTag, FromQueryResult,
    MergeList, TagList, TagsOfValueView, Value, ValueCount, ValueCountOf,
};
use crate::view::index::ResultIndex;

pub struct TupleExprView<L: Value, R: Value, LTags: TagList, RTags: TagList>(
    pub ExprViewBoxWithTag<L, LTags>,
    pub ExprViewBoxWithTag<R, RTags>,
)
where
    ValueCountOf<L>: Add<ValueCountOf<R>>,
    Sum<ValueCountOf<L>, ValueCountOf<R>>:
        ValueCount + Sub<ValueCountOf<L>, Output = ValueCountOf<R>>;

impl<L: Value, R: Value, LTags: TagList + MergeList<RTags>, RTags: TagList> ExprView<(L, R)>
    for TupleExprView<L, R, LTags, RTags>
where
    TagsOfValueView<L>: MergeList<TagsOfValueView<R>>,
    ValueCountOf<L>: Add<ValueCountOf<R>>,
    Sum<ValueCountOf<L>, ValueCountOf<R>>:
        ValueCount + Sub<ValueCountOf<L>, Output = ValueCountOf<R>>,
{
    type Tags = ConcreteList<LTags, RTags>;

    fn from_exprs(exprs: GenericArray<Expr, ValueCountOf<(L, R)>>) -> ExprViewBox<(L, R)>
    where
        Self: Sized,
    {
        let (v0, v1) = Split::split(exprs);
        Box::new(TupleExprView(
            L::view_from_exprs(v0),
            R::view_from_exprs(v1),
        ))
    }

    fn expr_clone(&self) -> ExprViewBoxWithTag<(L, R), Self::Tags> {
        Box::new(TupleExprView(self.0.expr_clone(), self.1.expr_clone()))
    }

    fn collect_expr(&self) -> GenericArray<Expr, ValueCountOf<(L, R)>> {
        Concat::concat(self.0.collect_expr(), self.1.collect_expr())
    }
}

impl<L: Value, R: Value> Value for (L, R)
where
    TagsOfValueView<L>: MergeList<TagsOfValueView<R>>,
    ValueCountOf<L>: Add<ValueCountOf<R>>,
    Sum<ValueCountOf<L>, ValueCountOf<R>>:
        ValueCount + Sub<ValueCountOf<L>, Output = ValueCountOf<R>>,
{
    type L = Sum<ValueCountOf<L>, ValueCountOf<R>>;
    type ValueExprView = TupleExprView<L, R, TagsOfValueView<L>, TagsOfValueView<R>>;

    fn to_database_values(self) -> GenericArray<DatabaseValue, Self::L> {
        Concat::concat(self.0.to_database_values(), self.1.to_database_values())
    }
}

impl<
        'r,
        DB: Database,
        H: ResultIndex,
        L: Value + FromQueryResult<'r, DB, H>,
        R: Value + FromQueryResult<'r, DB, Sum<ValueCountOf<L>, H>>,
    > FromQueryResult<'r, DB, H> for (L, R)
where
    TagsOfValueView<L>: MergeList<TagsOfValueView<R>>,
    ValueCountOf<L>: Add<ValueCountOf<R>> + ArrayLength<ColumnOf<DB>>,
    ValueCountOf<L>: Add<H>,
    Sum<ValueCountOf<L>, H>: ResultIndex,
    Sum<ValueCountOf<L>, ValueCountOf<R>>:
        ValueCount + Sub<ValueCountOf<L>, Output = ValueCountOf<R>>,
{
    fn from_result(values: &'r RowOf<DB>) -> EvalResult<Self>
    where
        Self: Sized,
    {
        Ok((L::from_result(values)?, R::from_result(values)?))
    }
}

impl<L: Value, R: Value, LTags: TagList, RTags: TagList>
    From<(ExprViewBoxWithTag<L, LTags>, ExprViewBoxWithTag<R, RTags>)>
    for TupleExprView<L, R, LTags, RTags>
where
    (L, R): Value,
    ValueCountOf<L>: Add<ValueCountOf<R>, Output = ValueCountOf<(L, R)>>,
    ValueCountOf<(L, R)>: Sub<ValueCountOf<L>, Output = ValueCountOf<R>>,
{
    fn from(tuple: (ExprViewBoxWithTag<L, LTags>, ExprViewBoxWithTag<R, RTags>)) -> Self {
        let (l, r) = tuple;
        TupleExprView(l, r)
    }
}
