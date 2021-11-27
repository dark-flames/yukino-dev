use std::marker::PhantomData;

use generic_array::typenum::U1;

use query_builder::{Function, FunctionCall};

use crate::view::{
    AggregateViewItem, AggregateViewTag, ExprViewBoxWithTag, TagList, TagList1, Value,
};

pub struct AggregateHelper(PhantomData<u8>);

pub(crate) trait AggregateHelperCreate {
    fn create() -> Self
    where
        Self: Sized;
}

impl AggregateHelperCreate for AggregateHelper {
    fn create() -> Self
    where
        Self: Sized,
    {
        AggregateHelper(PhantomData)
    }
}

macro_rules! impl_aggregate {
    (
        $trait_name: ident,
        $trait_method: ident,
        $method_name: ident,
        $macro_name: ident,
        $variant: ident,
        [$($types: ty),*]
    ) => {
        pub trait $trait_name: Value {
            type Result: Value<L=U1>;

            fn $trait_method<T: TagList>(expr: ExprViewBoxWithTag<Self, T>) -> AggregateViewItem<Self::Result>;
        }

        impl AggregateHelper {
            pub fn $method_name<V: Value + $trait_name, T: TagList>(
                &self,
                v: ExprViewBoxWithTag<V, T>,
            ) -> ExprViewBoxWithTag<<V as $trait_name>::Result, TagList1<AggregateViewTag>> {
                Box::new(V::$trait_method(v))
            }
        }

        macro_rules! $macro_name {
            ($ty: ty) => {
                impl $trait_name for $ty {
                    type Result = $ty;

                    fn $trait_method<T: TagList>(expr: ExprViewBoxWithTag<Self, T>) -> AggregateViewItem<Self::Result> {
                        AggregateViewItem::from_function_call(FunctionCall {
                            func: Function::$variant,
                            params: vec![expr.collect_expr().into_iter().next().unwrap()]
                        })
                    }
                }
            };
        }

        $($macro_name!($types);)*
    }
}

impl_aggregate!(
    ExprsAverage,
    exprs_average,
    average,
    impl_average,
    Average,
    [u16, i16, u32, i32, u64, i64, f32, f64]
);
impl_aggregate!(
    ExprsBitAnd,
    exprs_bit_and,
    bit_and,
    impl_bit_and,
    BitAnd,
    [u16, i16, u32, i32, u64, i64]
);
impl_aggregate!(
    ExprsBitOr,
    exprs_bit_or,
    bit_or,
    impl_bit_or,
    BitOr,
    [u16, i16, u32, i32, u64, i64]
);
impl_aggregate!(
    ExprsBitXor,
    exprs_bit_xor,
    bit_xor,
    impl_bit_xor,
    BitXor,
    [u16, i16, u32, i32, u64, i64]
);
impl_aggregate!(
    ExprsConcat,
    exprs_concat,
    concat,
    impl_concat,
    Concat,
    [String]
);
impl_aggregate!(
    ExprsMin,
    exprs_min,
    min,
    impl_min,
    Min,
    [u16, i16, u32, i32, u64, i64, f32, f64, char, String]
);
impl_aggregate!(
    ExprsMax,
    exprs_max,
    max,
    impl_max,
    Max,
    [u16, i16, u32, i32, u64, i64, f32, f64, char, String]
);

impl ExprsConcat for char {
    type Result = String;

    fn exprs_concat<T: TagList>(
        expr: ExprViewBoxWithTag<Self, T>,
    ) -> AggregateViewItem<Self::Result> {
        AggregateViewItem::from_function_call(FunctionCall {
            func: Function::Concat,
            params: vec![expr.collect_expr().into_iter().next().unwrap()],
        })
    }
}
