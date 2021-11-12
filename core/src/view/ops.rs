use crate::db::ty::DatabaseValue;
use crate::err::RuntimeResult;
use crate::query::Expr;
use crate::view::{
    ComputationView, ComputationViewBox, ExprView, ExprViewBox, SingleExprView, Value, ValueCount,
    View, ViewBox,
};
use generic_array::sequence::Split;
use generic_array::{arr, GenericArray};
use std::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Rem, Shl, Shr, Sub};

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

            fn $trait_method(
                l: ExprViewBox<Self, <Self as Value>::L>,
                r: ExprViewBox<Rhs, <Rhs as Value>::L>,
            ) -> ExprViewBox<Self::Result, <Self::Result as Value>::L>;
        }

        pub struct $computation_name<
            L: Value + $ops_trait<R, Output = O>,
            R: Value,
            O: 'static,
        > {
            l: ViewBox<L, <L as Value>::L>,
            r: ViewBox<R, <R as Value>::L>,
        }

        impl<
                L: Value + $ops_trait<R, Output = O>,
                R: Value,
                O: 'static,
                OL: ValueCount + Sub<<L as Value>::L, Output = <R as Value>::L>,
            > View<O, OL> for $computation_name<L, R, O>
        where <L as Value>::L: Add<<R as Value>::L, Output = OL>
        {
            fn eval(&self, v: &GenericArray<DatabaseValue, OL>) -> RuntimeResult<O> {
                let (l, r) = Split::<_, <L as Value>::L>::split(v);
                Ok(self.l.eval(l)? $op self.r.eval(r)?)
            }

            fn view_clone(&self) -> ViewBox<O, OL> {
                Box::new($computation_name {
                    l: self.l.view_clone(),
                    r: self.r.view_clone(),
                })
            }
        }

        impl<
                L: Value + $ops_trait<R, Output = O>,
                R: Value,
                O: 'static,
                OL: ValueCount + Sub<<L as Value>::L, Output = <R as Value>::L>,
            > ComputationView<O, OL> for $computation_name<L, R, O>
        where <L as Value>::L: Add<<R as Value>::L, Output = OL>
        {
            fn computation_clone(&self) -> ComputationViewBox<O, OL>
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
                L: Value + $expr_trait<R, Result = O>,
                R: Value,
                O: Value,
            > $ops_trait<ExprViewBox<R, <R as Value>::L>> for ExprViewBox<L, <L as Value>::L>
        {
            type Output = ExprViewBox<O, <O as Value>::L>;

            fn $ops_method(self, rhs: ExprViewBox<R, <R as Value>::L>) -> Self::Output {
                L::$trait_method(self, rhs)
            }
        }

        impl<
                L: Value + $expr_trait<R, Result = O>,
                R: Value,
                O: Value,
            > $ops_trait<R> for ExprViewBox<L, <L as Value>::L>
        {
            type Output = ExprViewBox<O, <O as Value>::L>;

            fn $ops_method(self, rhs: R) -> Self::Output {
                L::$trait_method(self, rhs.view())
            }
        }

        impl<
                L: Value + $expr_trait<R, Result = O>,
                R: Value,
                O: Value,

            > $ops_trait<ComputationViewBox<R, <R as Value>::L>> for ExprViewBox<L, <L as Value>::L>
        where <L as Value>::L: Add<<R as Value>::L, Output = <O as Value>::L>,
            <O as Value>::L: Sub<<L as Value>::L, Output = <R as Value>::L>
        {
            type Output = ComputationViewBox<O, <O as Value>::L>;

            fn $ops_method(self, rhs: ComputationViewBox<R, <R as Value>::L>) -> Self::Output {
                Box::new($computation_name {
                    l: self.view_clone(),
                    r: rhs.view_clone(),
                })
            }
        }

        impl<
                L: Value + $expr_trait<R, Result = O>,
                R: Value,
                O: Value
            > $ops_trait<R> for ComputationViewBox<L, <L as Value>::L>
        where <L as Value>::L: Add<<R as Value>::L, Output = <O as Value>::L>,
            <O as Value>::L: Sub<<L as Value>::L, Output = <R as Value>::L>
        {
            type Output = ComputationViewBox<O, <O as Value>::L>;

            fn $ops_method(self, rhs: R) -> Self::Output {
                Box::new($computation_name {
                    l: self.view_clone(),
                    r: rhs.view().view_clone(),
                })
            }
        }

        impl<
                L: Value + $expr_trait<R, Result = O>,
                R: Value,
                O: Value,

            > $ops_trait<ExprViewBox<R, <R as Value>::L>> for ComputationViewBox<L, <L as Value>::L>
        where <L as Value>::L: Add<<R as Value>::L, Output = <O as Value>::L>,
            <O as Value>::L: Sub<<L as Value>::L, Output = <R as Value>::L>,
        {
            type Output = ComputationViewBox<O, <O as Value>::L>;

            fn $ops_method(self, rhs: ExprViewBox<R, <R as Value>::L>) -> Self::Output {
                Box::new($computation_name {
                    l: self.view_clone(),
                    r: rhs.view_clone(),
                })
            }
        }

        impl<
                L: Value + $expr_trait<R, Result = O>,
                R: Value,
                O: Value,

            > $ops_trait<ComputationViewBox<R, <R as Value>::L>> for ComputationViewBox<L, <L as Value>::L>
        where <L as Value>::L: Add<<R as Value>::L, Output = <O as Value>::L>,
            <O as Value>::L: ValueCount + Sub<<L as Value>::L, Output = <R as Value>::L>,
        {
            type Output = ComputationViewBox<O, <O as Value>::L>;

            fn $ops_method(self, rhs: ComputationViewBox<R, <R as Value>::L>) -> Self::Output {
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

                    fn $trait_method(
                        l: ExprViewBox<Self, <Self as Value>::L>,
                        r: ExprViewBox<$ty, <$ty as Value>::L>,
                    ) -> ExprViewBox<Self::Result, <$ty as Value>::L> {
                        let l_expr = l.collect_expr().into_iter().next().unwrap();
                        let r_expr = r.collect_expr().into_iter().next().unwrap();
                        let result = Expr::$expr_variant(Box::new(l_expr), Box::new(r_expr));
                        Box::new(SingleExprView::from_exprs(arr![Expr; result]))
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
