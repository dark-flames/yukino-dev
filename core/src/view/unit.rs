use std::marker::PhantomData;

use generic_array::{arr, GenericArray};
use generic_array::typenum::U0;
use sqlx::Database;

use query_builder::{DatabaseValue, Expr, QueryOf, RowOf};

use crate::view::{
    AnyTagExprView, AnyTagsValue, ConvertResult, DBMapping, EmptyTagList, ExprView, ExprViewBox,
    ExprViewBoxWithTag, TagList, Value, ValueCountOf,
};
use crate::view::index::ResultIndex;

pub struct UnitView<Tags: TagList>(PhantomData<Tags>);

impl<Tags: TagList> ExprView<()> for UnitView<Tags> {
    type Tags = Tags;

    fn from_exprs(_exprs: GenericArray<Expr, ValueCountOf<()>>) -> ExprViewBox<()>
    where
        Self: Sized,
    {
        Box::new(UnitView::<EmptyTagList>(PhantomData))
    }

    fn expr_clone(&self) -> ExprViewBoxWithTag<(), Self::Tags> {
        Box::new(UnitView(PhantomData))
    }

    fn collect_expr(&self) -> GenericArray<Expr, ValueCountOf<()>> {
        arr![Expr;]
    }
}

impl<Tags: TagList> AnyTagExprView<()> for UnitView<Tags> {
    fn from_exprs_with_tags(
        _exprs: GenericArray<Expr, ValueCountOf<()>>,
    ) -> ExprViewBoxWithTag<(), Self::Tags>
    where
        Self: Sized,
    {
        Box::new(UnitView::<Tags>(PhantomData))
    }
}

impl Value for () {
    type L = U0;
    type ValueExprView = UnitView<EmptyTagList>;

    fn to_database_values(self) -> GenericArray<DatabaseValue, Self::L> {
        arr![DatabaseValue;]
    }
}

impl<'r, DB: Database, H: ResultIndex> DBMapping<'r, DB, H> for () {
    fn from_result(_values: &'r RowOf<DB>) -> ConvertResult<Self>
    where
        Self: Sized,
    {
        Ok(())
    }

    fn bind_on_query(self, query: QueryOf<DB>) -> QueryOf<DB>
    where
        Self: Sized,
    {
        query
    }
}

impl AnyTagsValue for () {
    fn view_with_tags<Tags: TagList>(self) -> ExprViewBoxWithTag<Self, Tags> {
        Box::new(UnitView::<Tags>(PhantomData))
    }
}
