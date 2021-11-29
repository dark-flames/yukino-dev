use std::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Rem, Shl, Shr, Sub};

use generic_array::arr;

use query_builder::Expr;

use crate::view::{ExprView, ExprViewBoxWithTag, SingleExprView, TagList, TagOfValueView, Value};

macro_rules! impl_ops {
    (
        $op: tt,
        $expr_trait: ident,
        $trait_method: ident,
        $ops_trait: ident,
        $ops_method: ident,
        $expr_variant: ident,
        $macro_name: ident,
        [$($impl_tys: ty),*]
    ) => {
        pub trait $expr_trait<Rhs: Value>:
            Value + $ops_trait<Rhs, Output = Self::Result>
        {
            type Result: Value;
            type ResultTags<LTags: TagList, RTags: TagList>: TagList;

            fn $trait_method<LTags: TagList, RTags: TagList>(
                l: ExprViewBoxWithTag<Self, LTags>,
                r: ExprViewBoxWithTag<Rhs, RTags>,
            ) -> ExprViewBoxWithTag<Self::Result, Self::ResultTags<LTags, RTags>>;
        }

        impl<
            L: Value + $expr_trait<R, Result = O, ResultTags<LTags, RTags> = OTags>,
            R: Value,
            O: Value,
            LTags: TagList,
            RTags: TagList,
            OTags: TagList
        > $ops_trait<ExprViewBoxWithTag<R, RTags>> for ExprViewBoxWithTag<L, LTags>
        {
            type Output = ExprViewBoxWithTag<O, OTags>;

            fn $ops_method(self, rhs: ExprViewBoxWithTag<R, RTags>) -> Self::Output {
                L::$trait_method(self, rhs)
            }
        }

        impl<
            L: Value + $expr_trait<R, Result = O, ResultTags<LTags, TagOfValueView<R>> = OTags>,
            R: Value,
            O: Value,
            LTags: TagList,
            OTags: TagList
        > $ops_trait<R> for ExprViewBoxWithTag<L, LTags>
        {
            type Output = ExprViewBoxWithTag<O, OTags>;

            fn $ops_method(self, rhs: R) -> Self::Output {
                L::$trait_method(self, rhs.view())
            }
        }

        macro_rules! $macro_name {
            ($ty: ty) => {
                impl $expr_trait<$ty> for $ty {
                    type Result = $ty;
                    type ResultTags<LTags: TagList, RTags: TagList> = TagOfValueView<$ty>;

                    fn $trait_method<LTags: TagList, RTags: TagList>(
                        l: ExprViewBoxWithTag<Self, LTags>,
                        r: ExprViewBoxWithTag<$ty, RTags>,
                    ) -> ExprViewBoxWithTag<Self::Result, Self::ResultTags<LTags, RTags>>{
                        let l_expr = l.collect_expr().into_iter().next().unwrap();
                        let r_expr = r.collect_expr().into_iter().next().unwrap();
                        let result = Expr::$expr_variant(Box::new(l_expr), Box::new(r_expr));
                        SingleExprView::from_exprs(arr![Expr; result])
                    }
                }
            }
        }
        $($macro_name!($impl_tys);)*
    };
}

impl_ops!(+, ExprAdd, expr_add, Add, add, Add, impl_add, [
    u16, i16, u32, i32, u64, i64, f32, f64]);
impl_ops!(-, ExprSub, expr_sub, Sub, sub, Sub, impl_sub, [
    u16, i16, u32, i32, u64, i64, f32, f64]);
impl_ops!(*, ExprMul, expr_mul, Mul, mul, Mul, impl_mul, [
    u16, i16, u32, i32, u64, i64, f32, f64]);
impl_ops!(/, ExprDiv, expr_div, Div, div, Div, impl_div, [
    u16, i16, u32, i32, u64, i64, f32, f64]);
impl_ops!(%, ExprRem, expr_rem, Rem, rem, Rem, impl_rem, [
    u16, i16, u32, i32, u64, i64]);
impl_ops!(<<, ExprLeftShift, expr_left_shift, Shl, shl, LeftShift,
    impl_left_shift, [u16, i16, u32, i32, u64, i64]);
impl_ops!(>>, ExprRightShift, expr_right_shift, Shr, shr, RightShift,
    impl_right_shift, [u16, i16, u32, i32, u64, i64]);
impl_ops!(&, ExprBitAnd, expr_bit_and, BitAnd, bitand, BitAnd,
    impl_bit_and, [u16, i16, u32, i32, u64, i64]);
impl_ops!(|, ExprBitOr, expr_bit_or, BitOr, bitor, BitOr,
    impl_bit_or, [u16, i16, u32, i32, u64, i64]);
impl_ops!(^, ExprBitXor, expr_bit_xor, BitXor, bitxor, BitXor,
    impl_bit_xor, [u16, i16, u32, i32, u64, i64]);
