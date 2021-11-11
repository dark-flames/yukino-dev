use crate::converter::basic::*;
use crate::converter::{Converter, ConverterRef};
use crate::db::ty::DatabaseValue;
use crate::err::{RuntimeResult, YukinoError};
use crate::query::Expr;
use crate::view::{ExprView, ExprViewBox, ValueView, View, ViewBox};
use generic_array::typenum::bit::{B0, B1};
use generic_array::typenum::{UInt, UTerm, U1};
use generic_array::{arr, functional::FunctionalSequence, ArrayLength, GenericArray};
use std::fmt::Debug;
use std::marker::PhantomData;

pub trait ValueCount: ArrayLength<Expr> + ArrayLength<DatabaseValue> {}

impl ValueCount for UTerm {}
impl<N: ValueCount> ValueCount for UInt<N, B0> {}
impl<N: ValueCount> ValueCount for UInt<N, B1> {}

pub trait Value: 'static + Clone + Debug {
    type L: ValueCount;
    fn converter() -> ConverterRef<Self, Self::L>
    where
        Self: Sized;

    fn view(&self) -> ExprViewBox<Self, Self::L>
    where
        Self: Sized;
}

#[derive(Debug, Clone)]
pub struct SingleExprView<T>
where
    T: Value<L = U1>,
{
    expr: Expr,
    _ty: PhantomData<T>,
}

impl<T> View<T, U1> for SingleExprView<T>
where
    T: Value<L = U1>,
{
    fn eval(&self, v: &GenericArray<DatabaseValue, U1>) -> RuntimeResult<T> {
        (*T::converter().deserializer())(v).map_err(|e| e.as_runtime_err())
    }

    fn view_clone(&self) -> ViewBox<T, U1> {
        Box::new(Clone::clone(self))
    }
}

impl<T> ValueView<T, U1> for SingleExprView<T>
where
    T: Value<L = U1>,
{
    fn collect_expr(&self) -> GenericArray<Expr, U1> {
        arr![Expr; self.expr.clone()]
    }
}

impl<T> ExprView<T, U1> for SingleExprView<T>
where
    T: Value<L = U1>,
{
    fn from_exprs(exprs: GenericArray<Expr, U1>) -> Self
    where
        Self: Sized,
    {
        SingleExprView {
            expr: exprs.into_iter().next().unwrap(),
            _ty: Default::default(),
        }
    }

    fn expr_clone(&self) -> ExprViewBox<T, U1>
    where
        Self: Sized,
    {
        Box::new(Clone::clone(self))
    }
}

macro_rules! impl_value {
    ($ty: ty, $converter: ty) => {
        impl Value for $ty {
            type L = U1;
            fn converter() -> ConverterRef<Self, Self::L>
            where
                Self: Sized,
            {
                <$converter>::instance()
            }

            fn view(&self) -> ExprViewBox<Self, Self::L>
            where
                Self: Sized,
            {
                Box::new(SingleExprView::<$ty>::from_exprs(
                    Self::converter().serialize(self).unwrap().map(Expr::Lit),
                ))
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
