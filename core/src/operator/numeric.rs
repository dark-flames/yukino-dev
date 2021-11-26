use std::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Rem, Shl, Shr, Sub};

use generic_array::{arr, GenericArray};
use generic_array::sequence::{Concat, Split};
use generic_array::typenum::operator_aliases::Sum;

use query_builder::{DatabaseValue, Expr};

use crate::err::RuntimeResult;
use crate::query::{ExprMutVisitor, ExprNode, ExprVisitor};
use crate::view::{
    ComputationView, ComputationViewBox, ExprView, ExprViewBoxWithTag, SingleExprView, TagList,
    TagOfValueView, Value, ValueCount, ValueCountOf, View, ViewBox,
};

macro_rules! impl_ops {
    (
        $op: tt,
        $expr_trait: ident,
        $trait_method: ident,
        $computation_name: ident,
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
            type ResultTags: TagList;

            fn $trait_method<LTags: TagList, RTags: TagList>(
                l: ExprViewBoxWithTag<Self, LTags>,
                r: ExprViewBoxWithTag<Rhs, RTags>,
            ) -> ExprViewBoxWithTag<Self::Result, Self::ResultTags>;
        }

        pub struct $computation_name<
            L: 'static + $ops_trait<R, Output = O>,
            R: 'static,
            O: 'static,
            LL: ValueCount,
            RL: ValueCount
        > {
            l: ViewBox<L, LL>,
            r: ViewBox<R, RL>,
        }

        impl<
            L: 'static + $ops_trait<R, Output = O>,
            R: 'static,
            O: 'static,
            LL: ValueCount + Add<RL>,
            RL: ValueCount,
        > ExprNode for $computation_name<L, R, O, LL, RL>
        where Sum<LL, RL>: ValueCount + Sub<LL, Output=RL>
        {
            fn apply(&self, visitor: &mut dyn ExprVisitor) {
                self.l.apply(visitor);
                self.r.apply(visitor);
            }

            fn apply_mut(&mut self, visitor: &mut dyn ExprMutVisitor) {
                self.l.apply_mut(visitor);
                self.r.apply_mut(visitor);
            }
        }

        impl<
            L: 'static + $ops_trait<R, Output = O>,
            R: 'static,
            O: 'static,
            LL: ValueCount + Add<RL>,
            RL: ValueCount
        > View<O, Sum<LL, RL>> for $computation_name<L, R, O, LL, RL>
        where Sum<LL, RL>: ValueCount + Sub<LL, Output=RL>
        {
            fn collect_expr(&self) -> GenericArray<Expr, Sum<LL, RL>> {
                Concat::concat(self.l.collect_expr(), self.r.collect_expr())
            }

            fn eval(&self, v: &GenericArray<DatabaseValue, Sum<LL, RL>>) -> RuntimeResult<O> {
                let (l, r) = Split::<_, LL>::split(v);
                Ok(self.l.eval(l)? $op self.r.eval(r)?)
            }

            fn view_clone(&self) -> ViewBox<O, Sum<LL, RL>> {
                Box::new($computation_name {
                    l: self.l.view_clone(),
                    r: self.r.view_clone(),
                })
            }
        }

        impl<
            L: 'static + $ops_trait<R, Output = O>,
            R: 'static,
            O: 'static,
            LL: ValueCount + Add<RL>,
            RL: ValueCount,
        > ComputationView<O, Sum<LL, RL>> for $computation_name<L, R, O, LL, RL>
        where Sum<LL, RL>: ValueCount + Sub<LL, Output=RL>
        {
            fn computation_clone(&self) -> ComputationViewBox<O, Sum<LL, RL>>
            where
                Self: Sized,
            {
                Box::new($computation_name {
                    l: self.l.view_clone(),
                    r: self.r.view_clone(),
                })
            }
        }

        impl<
            L: Value + $expr_trait<R, Result = O, ResultTags = OTags>,
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
            L: Value + $expr_trait<R, Result = O, ResultTags = OTags>,
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

        impl<
            L: Value + $ops_trait<R, Output = O>,
            R: 'static,
            O: 'static,
            RL: ValueCount,
            LTags: TagList,
        > $ops_trait<ComputationViewBox<R, RL>> for ExprViewBoxWithTag<L, LTags>
        where ValueCountOf<L>: Add<RL>,
            Sum<ValueCountOf<L>, RL>: ValueCount + Sub<ValueCountOf<L>, Output = RL>
        {
            type Output = ComputationViewBox<O, Sum<ValueCountOf<L>, RL>>;

            fn $ops_method(self, rhs: ComputationViewBox<R, RL>) -> Self::Output {
                Box::new($computation_name {
                    l: self.view_clone(),
                    r: rhs.view_clone(),
                })
            }
        }

        impl<
            L: 'static + $ops_trait<R, Output = O>,
            R: Value,
            O: 'static,
            LL: ValueCount + Add<ValueCountOf<R>>
        > $ops_trait<R> for ComputationViewBox<L, LL>
        where Sum<LL, ValueCountOf<R>>: ValueCount + Sub<LL, Output = ValueCountOf<R>>
        {
            type Output = ComputationViewBox<O, Sum<LL, ValueCountOf<R>>>;

            fn $ops_method(self, rhs: R) -> Self::Output {
                Box::new($computation_name {
                    l: self.view_clone(),
                    r: rhs.view().view_clone(),
                })
            }
        }

        impl<
            L: 'static + $ops_trait<R, Output = O>,
            R: Value,
            O: 'static,
            LL: ValueCount + Add<ValueCountOf<R>>,
            RTags: TagList,
        > $ops_trait<ExprViewBoxWithTag<R, RTags>> for ComputationViewBox<L, LL>
        where Sum<LL, ValueCountOf<R>>: ValueCount + Sub<LL, Output = ValueCountOf<R>>
        {
            type Output = ComputationViewBox<O, Sum<LL, ValueCountOf<R>>>;

            fn $ops_method(self, rhs: ExprViewBoxWithTag<R, RTags>) -> Self::Output {
                Box::new($computation_name {
                    l: self.view_clone(),
                    r: rhs.view_clone(),
                })
            }
        }

        impl<
            L: 'static + $ops_trait<R, Output = O>,
            R: 'static,
            O: 'static,
            LL: ValueCount + Add<RL>,
            RL: ValueCount
        > $ops_trait<ComputationViewBox<R, RL>> for ComputationViewBox<L, LL>
        where Sum<LL, RL>: ValueCount + Sub<LL, Output = RL>
        {
            type Output = ComputationViewBox<O, Sum<LL, RL>>;

            fn $ops_method(self, rhs: ComputationViewBox<R, RL>) -> Self::Output {
                Box::new($computation_name {
                    l: self.view_clone(),
                    r: rhs.view_clone(),
                })
            }
        }

        macro_rules! $macro_name {
            ($ty: ty) => {
                impl $expr_trait<$ty> for $ty {
                    type Result = $ty;
                    type ResultTags = TagOfValueView<$ty>;

                    fn $trait_method<LTags: TagList, RTags: TagList>(
                        l: ExprViewBoxWithTag<Self, LTags>,
                        r: ExprViewBoxWithTag<$ty, RTags>,
                    ) -> ExprViewBoxWithTag<Self::Result, Self::ResultTags>{
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

impl_ops!(+, ExprAdd, expr_add, AddView, Add, add, Add, impl_add, [
    u16, i16, u32, i32, u64, i64, f32, f64]);
impl_ops!(-, ExprSub, expr_sub, SubView, Sub, sub, Sub, impl_sub, [
    u16, i16, u32, i32, u64, i64, f32, f64]);
impl_ops!(*, ExprMul, expr_mul, MulView, Mul, mul, Mul, impl_mul, [
    u16, i16, u32, i32, u64, i64, f32, f64]);
impl_ops!(/, ExprDiv, expr_div, DivView, Div, div, Div, impl_div, [
    u16, i16, u32, i32, u64, i64, f32, f64]);
impl_ops!(%, ExprRem, expr_rem, RemView, Rem, rem, Rem, impl_rem, [
    u16, i16, u32, i32, u64, i64]);
impl_ops!(<<, ExprLeftShift, expr_left_shift, LeftShiftView, Shl, shl, LeftShift,
    impl_left_shift, [u16, i16, u32, i32, u64, i64]);
impl_ops!(>>, ExprRightShift, expr_right_shift, RightShiftView, Shr, shr, RightShift,
    impl_right_shift, [u16, i16, u32, i32, u64, i64]);
impl_ops!(&, ExprBitAnd, expr_bit_and, BitAndView, BitAnd, bitand, BitAnd,
    impl_bit_and, [u16, i16, u32, i32, u64, i64]);
impl_ops!(|, ExprBitOr, expr_bit_or, BitOrView, BitOr, bitor, BitOr,
    impl_bit_or, [u16, i16, u32, i32, u64, i64]);
impl_ops!(^, ExprBitXor, expr_bit_xor, BitXorView, BitXor, bitxor, BitXor,
    impl_bit_xor, [u16, i16, u32, i32, u64, i64]);
