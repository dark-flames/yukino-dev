use query_builder::{
    AggregateFunction, GroupConcatFunctionCall, NormalAggregateFunctionCall, OrderByItem,
};

use crate::view::{
    AddTag, AggregateViewItem, AggregateViewTag, ExprViewBoxWithTag, OffsetOfTag, SetBit, TagList,
    TagsOfValueView, True, Value, VerticalExprView, VerticalView,
};

pub trait ExprJoin {
    fn expr_join<TTags: TagList>(
        expr: ExprViewBoxWithTag<Self, TTags>,
        order_by_items: Vec<OrderByItem>,
        separator: Option<&str>,
    ) -> ExprViewBoxWithTag<String, AddTag<TagsOfValueView<String>, AggregateViewTag>>;
}

pub trait VerticalJoin {
    fn vertical_join(
        self,
        separator: Option<&str>,
    ) -> ExprViewBoxWithTag<String, AddTag<TagsOfValueView<String>, AggregateViewTag>>;
}

impl<T: Value + ExprJoin, TTags: TagList + SetBit<OffsetOfTag<AggregateViewTag>, True>> VerticalJoin
    for VerticalExprView<T, TTags>
{
    fn vertical_join(
        self,
        separator: Option<&str>,
    ) -> ExprViewBoxWithTag<String, AddTag<TagsOfValueView<String>, AggregateViewTag>> {
        T::expr_join(self.expr, self.order_by, separator)
    }
}

macro_rules! impl_join_for {
    [$($ty: ty),*] => {
        $(
            impl ExprJoin for $ty {
                fn expr_join<TTags: TagList>(
                    expr: ExprViewBoxWithTag<Self, TTags>,
                    order_by_items: Vec<OrderByItem>,
                    separator: Option<&str>,
                ) -> ExprViewBoxWithTag<String, AddTag<TagsOfValueView<String>, AggregateViewTag>>
                {
                    Box::new(AggregateViewItem::<String, TagsOfValueView<String>>::from_agg_fn_call(GroupConcatFunctionCall {
                        expr: expr.collect_expr().into_iter().next().unwrap(),
                        order_by: order_by_items,
                        separator: separator.map(ToString::to_string),
                    }))
                }
            }
        )*
    };
}

macro_rules! impl_aggr_fn {
    (@internal $trait_name: ident, $trait_method: ident, $return_ty: ty, $return_base_tags: ty, $vertical_trait: ident, $vertical_method: ident, $vertical_return_ty: ty, $variant: ident, [$($ty: ty),*]) => {
        pub trait $trait_name: Value
        {
            fn $trait_method<TTags: TagList>(
                expr: ExprViewBoxWithTag<Self, TTags>,
            ) -> ExprViewBoxWithTag<$return_ty, AddTag<$return_base_tags, AggregateViewTag>>
                where $return_base_tags: SetBit<OffsetOfTag<AggregateViewTag>, True>;
        }

        pub trait $vertical_trait<
            T: Value + $trait_name,
            TTags: TagList
        >: VerticalView<T>
            where $return_base_tags: SetBit<OffsetOfTag<AggregateViewTag>, True>
        {
            fn $vertical_method(self) -> ExprViewBoxWithTag<$vertical_return_ty, AddTag<$return_base_tags, AggregateViewTag>>;
        }

        impl<
            T: Value + $trait_name,
            TTags: TagList + SetBit<OffsetOfTag<AggregateViewTag>, True>
        > $vertical_trait<T, TTags> for VerticalExprView<T, TTags>
        where $return_base_tags: SetBit<OffsetOfTag<AggregateViewTag>, True>
        {
            fn $vertical_method(self) -> ExprViewBoxWithTag<$vertical_return_ty, AddTag<$return_base_tags, AggregateViewTag>> {
                T::$trait_method(self.expr)
            }
        }

        $(
            impl $trait_name for $ty {
                fn $trait_method<TTags: TagList>(
                    expr: ExprViewBoxWithTag<Self, TTags>,
                ) -> ExprViewBoxWithTag<$return_ty, AddTag<$return_base_tags, AggregateViewTag>>
                where $return_base_tags: SetBit<OffsetOfTag<AggregateViewTag>, True>
                {
                    Box::new(AggregateViewItem::<$return_ty, $return_base_tags>::from_agg_fn_call(NormalAggregateFunctionCall {
                        function: AggregateFunction::$variant,
                        param: expr.collect_expr().into_iter().next().unwrap()
                    }))
                }
            }
        )*
    };
    ($trait_name: ident, $trait_method: ident, $return_ty: ty, $return_tags: ty, $vertical_trait: ident, $vertical_method: ident, $variant: ident, [$($ty: ty),*]) => {
        impl_aggr_fn!(@internal $trait_name, $trait_method, $return_ty, $return_tags, $vertical_trait, $vertical_method, $return_ty, $variant, [$($ty),*]);
    };
    ($trait_name: ident, $trait_method: ident, $vertical_trait: ident, $vertical_method: ident, $variant: ident, [$($ty: ty),*]) => {
        impl_aggr_fn!(@internal $trait_name, $trait_method, Self, TTags, $vertical_trait, $vertical_method, T, $variant, [$($ty),*]);
    };
}

impl_aggr_fn!(
    ExprAverage,
    expr_average,
    VerticalAverage,
    average,
    Average,
    [u16, i16, u32, i32, u64, i64, f32, f64]
);
impl_aggr_fn!(
    ExprBitAnd,
    expr_bit_and,
    VerticalBitAnd,
    bit_and,
    BitAnd,
    [bool, u16, i16, u32, i32, u64, i64]
);
impl_aggr_fn!(
    ExprBitOr,
    expr_bit_or,
    VerticalBitOr,
    bit_or,
    BitOr,
    [bool, u16, i16, u32, i32, u64, i64]
);
impl_aggr_fn!(
    ExprBitXor,
    expr_bit_xor,
    VerticalBitXor,
    bit_xor,
    BitXor,
    [bool, u16, i16, u32, i32, u64, i64]
);
impl_aggr_fn!(
    ExprCount,
    expr_count,
    u64,
    TagsOfValueView<u64>,
    VerticalCount,
    count,
    Count,
    [bool, u16, i16, u32, i32, u64, i64, f32, f64, char, String]
);
impl_aggr_fn!(
    ExprCountDistinct,
    expr_count_distinct,
    u64,
    TagsOfValueView<u64>,
    VerticalCountDistinct,
    count_distinct,
    CountDistinct,
    [bool, u16, i16, u32, i32, u64, i64, f32, f64, char, String]
);
impl_aggr_fn!(
    ExprMax,
    expr_max,
    VerticalMax,
    max,
    Max,
    [u16, i16, u32, i32, u64, i64, f32, f64, char, String]
);
impl_aggr_fn!(
    ExprMin,
    expr_min,
    VerticalMin,
    min,
    Min,
    [u16, i16, u32, i32, u64, i64, f32, f64, char, String]
);

impl_join_for![bool, u16, i16, u32, i32, u64, i64, f32, f64, char, String];
