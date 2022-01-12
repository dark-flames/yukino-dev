use std::fmt::Debug;
use std::marker::PhantomData;

use generic_array::{arr, ArrayLength, functional::FunctionalSequence, GenericArray};
use generic_array::typenum::{U1, UInt, UTerm};
use generic_array::typenum::bit::{B0, B1};
use sqlx::{ColumnIndex, Database, Decode, MySql, Row};
use sqlx::types::Decimal;
use sqlx::types::time::{Date, PrimitiveDateTime, Time};

use interface::DatabaseType;
use query_builder::{DatabaseValue, Expr, RowOf};

use crate::view::{
    AnyTagExprView, EvalResult, ExprView, ExprViewBox, ExprViewBoxWithTag, OrdViewTag, TagList,
    TagList1,
};
use crate::view::index::ResultIndex;

pub type ValueCountOf<T> = <T as Value>::L;

pub trait ValueCount:
    ArrayLength<Expr> + ArrayLength<DatabaseValue> + ArrayLength<String> + ArrayLength<MySql>
{
}

impl ValueCount for UTerm {}

impl<N: ValueCount> ValueCount for UInt<N, B0> {}

impl<N: ValueCount> ValueCount for UInt<N, B1> {}

pub trait Value: 'static + Clone + Debug + Send + Sync {
    type L: ValueCount;
    type ValueExprView: ExprView<Self>;

    fn view(self) -> ExprViewBox<Self>
    where
        Self: Sized,
    {
        Self::view_from_exprs(self.to_database_values().map(Expr::Lit))
    }

    fn view_from_exprs(exprs: GenericArray<Expr, Self::L>) -> ExprViewBox<Self>
    where
        Self: Sized,
    {
        Self::ValueExprView::from_exprs(exprs)
    }

    fn to_database_values(self) -> GenericArray<DatabaseValue, Self::L>;
}

pub trait FromQueryResult<'r, DB: Database, H: ResultIndex>: Value {
    fn from_result(values: &'r RowOf<DB>) -> EvalResult<Self>
    where
        Self: Sized;
}

pub trait AnyTagsValue: Value {
    fn view_with_tags<Tags: TagList>(self) -> ExprViewBoxWithTag<Self, Tags>;
}

#[derive(Debug, Clone)]
pub struct SingleExprView<T: Value<L = U1>, Tags: TagList> {
    expr: Expr,
    _ty: PhantomData<(T, Tags)>,
}

impl<T: Value<L = U1>, Tags: TagList> ExprView<T> for SingleExprView<T, Tags> {
    type Tags = Tags;

    fn from_exprs(exprs: GenericArray<Expr, U1>) -> ExprViewBox<T>
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
}

impl<T: Value<L = U1>, Tags: TagList> AnyTagExprView<T> for SingleExprView<T, Tags> {
    fn from_exprs_with_tags(
        exprs: GenericArray<Expr, ValueCountOf<T>>,
    ) -> ExprViewBoxWithTag<T, Self::Tags>
    where
        Self: Sized,
    {
        Box::new(SingleExprView {
            expr: exprs.into_iter().next().unwrap(),
            _ty: Default::default(),
        })
    }
}

macro_rules! impl_value {
    (@inner $ty: ty, $enum: ident) => {
        impl<'r, DB: Database, H: ResultIndex> FromQueryResult<'r, DB, H> for $ty where
            Self: Decode<'r, DB>,
            for<'n> &'n str: ColumnIndex<RowOf<DB>>
        {
            fn from_result(
                values: &'r RowOf<DB>
            ) -> EvalResult<Self>
                where Self: Sized
            {
                values.try_get_unchecked(H::index())
            }
        }

        impl AnyTagsValue for $ty {
            fn view_with_tags<Tags: TagList>(self) -> ExprViewBoxWithTag<Self, Tags> {
                Box::new(SingleExprView {
                    expr: self.to_database_values()
                        .map(Expr::Lit)
                        .into_iter()
                        .next()
                        .unwrap(),
                    _ty: Default::default(),
                })
            }
        }
    };

    ($ty: ty, $enum: ident) => {
        impl Value for $ty {
            type L = U1;
            type ValueExprView = SingleExprView<Self, TagList1<OrdViewTag>>;

            fn to_database_values(self) -> GenericArray<DatabaseValue, Self::L>{
                arr![DatabaseValue; DatabaseValue::$enum(self)]
            }
        }

        impl Value for Option<$ty> {
            type L = U1;
            type ValueExprView = SingleExprView<Self, TagList1<OrdViewTag>>;

            fn to_database_values(self) -> GenericArray<DatabaseValue, Self::L>{
                if let Some(nested) = self {
                    arr![DatabaseValue; DatabaseValue::$enum(nested)]
                } else {
                    arr![DatabaseValue; DatabaseValue::Null(DatabaseType::$enum)]
                }
            }
        }

        impl_value!(@inner $ty, $enum);
        impl_value!(@inner Option<$ty>, $enum);
    };
}

impl_value!(bool, Bool);
impl_value!(u16, UnsignedSmallInteger);
impl_value!(u32, UnsignedInteger);
impl_value!(u64, UnsignedBigInteger);
impl_value!(i16, SmallInteger);
impl_value!(i32, Integer);
impl_value!(i64, BigInteger);
impl_value!(f32, Float);
impl_value!(f64, Double);
impl_value!(Decimal, Decimal);
impl_value!(Date, Date);
impl_value!(Time, Time);
impl_value!(PrimitiveDateTime, DateTime);
impl_value!(String, String);
