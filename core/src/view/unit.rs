use std::marker::PhantomData;

use generic_array::{arr, GenericArray};
use generic_array::typenum::U0;

use query_builder::{DatabaseValue, Expr};

use crate::converter::{Converter, ConverterRef, UnitConverter};
use crate::err::{RuntimeResult, YukinoError};
use crate::view::{AnyTagExprView, AnyTagsValue, EmptyTagList, ExprView, ExprViewBox, ExprViewBoxWithTag, TagList, Value, ValueCountOf};

pub struct UnitView<Tags: TagList>(PhantomData<Tags>);


impl<Tags: TagList> ExprView<()> for UnitView<Tags> {
    type Tags = Tags;

    fn from_exprs(_exprs: GenericArray<Expr, ValueCountOf<()>>) -> ExprViewBox<()> where Self: Sized {
        Box::new(UnitView::<EmptyTagList>(PhantomData))
    }

    fn expr_clone(&self) -> ExprViewBoxWithTag<(), Self::Tags> {
        Box::new(UnitView(PhantomData))
    }

    fn collect_expr(&self) -> GenericArray<Expr, ValueCountOf<()>> {
        arr![Expr;]
    }

    fn eval(&self, v: &GenericArray<DatabaseValue, ValueCountOf<()>>) -> RuntimeResult<()> {
        (*<() as Value>::converter().deserializer())(v).map_err(|e| e.as_runtime_err())
    }
}

impl<Tags: TagList> AnyTagExprView<()> for UnitView<Tags> {
    fn from_exprs_with_tags(_exprs: GenericArray<Expr, ValueCountOf<()>>) -> ExprViewBoxWithTag<(), Self::Tags> where Self: Sized {
        Box::new(UnitView::<Tags>(PhantomData))
    }
}

impl Value for () {
    type L = U0;
    type ValueExprView = UnitView<EmptyTagList>;

    fn converter() -> ConverterRef<Self> where Self: Sized {
        UnitConverter::instance()
    }
}

impl AnyTagsValue for () {
    fn view_with_tags<Tags: TagList>(&self) -> ExprViewBoxWithTag<Self, Tags> {
        Box::new(UnitView::<Tags>(PhantomData))
    }
}