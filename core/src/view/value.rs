use std::fmt::Debug;
use std::marker::PhantomData;

use generic_array::{arr, ArrayLength, functional::FunctionalSequence, GenericArray};
use generic_array::typenum::{U1, UInt, UTerm};
use generic_array::typenum::bit::{B0, B1};

use query_builder::{DatabaseValue, Expr};

use crate::converter::*;
use crate::err::{RuntimeResult, YukinoError};
use crate::view::{ExprView, ExprViewBox, ExprViewBoxWithTag, OrdViewTag, TagList, TagList1};

pub type ValueCountOf<T> = <T as Value>::L;

pub trait ValueCount: ArrayLength<Expr> + ArrayLength<DatabaseValue> {}

impl ValueCount for UTerm {}

impl<N: ValueCount> ValueCount for UInt<N, B0> {}

impl<N: ValueCount> ValueCount for UInt<N, B1> {}

pub trait Value: 'static + Clone + Debug {
    type L: ValueCount;
    type ValueExprView: ExprView<Self>;

    fn converter() -> ConverterRef<Self>
    where
        Self: Sized;

    fn view(&self) -> ExprViewBox<Self>
    where
        Self: Sized,
    {
        Self::view_from_exprs(Self::converter().serialize(self).unwrap().map(Expr::Lit))
    }

    fn view_from_exprs(
        exprs: GenericArray<Expr, Self::L>,
    ) -> ExprViewBoxWithTag<Self, <Self::ValueExprView as ExprView<Self>>::Tags>
    where
        Self: Sized,
    {
        Self::ValueExprView::from_exprs(exprs)
    }
}

#[derive(Debug, Clone)]
pub struct SingleExprView<T: Value<L = U1>, Tags: TagList> {
    expr: Expr,
    _ty: PhantomData<(T, Tags)>,
}

impl<T: Value<L = U1>, Tags: TagList> ExprView<T> for SingleExprView<T, Tags> {
    type Tags = Tags;

    fn from_exprs(exprs: GenericArray<Expr, U1>) -> ExprViewBoxWithTag<T, Self::Tags>
    where
        Self: Sized,
    {
        Box::new(SingleExprView {
            expr: exprs.into_iter().next().unwrap(),
            _ty: Default::default(),
        })
    }

    fn expr_clone(&self) -> ExprViewBoxWithTag<T, Self::Tags>
    where
        Self: Sized,
    {
        Box::new(SingleExprView {
            expr: self.expr.clone(),
            _ty: Default::default(),
        })
    }

    fn collect_expr(&self) -> GenericArray<Expr, U1> {
        arr![Expr; self.expr.clone()]
    }

    fn eval(&self, v: &GenericArray<DatabaseValue, U1>) -> RuntimeResult<T> {
        (*T::converter().deserializer())(v).map_err(|e| e.as_runtime_err())
    }
}

macro_rules! impl_value {
    ($ty: ty, $converter: ty) => {
        impl Value for $ty {
            type L = U1;
            type ValueExprView = SingleExprView<$ty, TagList1<OrdViewTag>>;

            fn converter() -> ConverterRef<Self>
            where
                Self: Sized,
            {
                <$converter>::instance()
            }
        }
    };

    ($ty: ty, $converter: ty, $optional_converter: ty) => {
        impl_value!($ty, $converter);
        impl_value!(Option<$ty>, $optional_converter);
    };
}

impl_value!(bool, BoolConverter, OptionalBoolConverter);
impl_value!(u16, UnsignedShortConverter, OptionalUnsignedShortConverter);
impl_value!(u32, UnsignedIntConverter, OptionalUnsignedIntConverter);
impl_value!(u64, UnsignedLongConverter, OptionalUnsignedLongConverter);
impl_value!(i16, ShortConverter, OptionalShortConverter);
impl_value!(i32, IntConverter, OptionalIntConverter);
impl_value!(i64, LongConverter, OptionalLongConverter);
impl_value!(f32, FloatConverter, OptionalFloatConverter);
impl_value!(f64, DoubleConverter, OptionalDoubleConverter);
impl_value!(char, CharConverter, OptionalCharConverter);
impl_value!(String, StringConverter, OptionalStringConverter);
