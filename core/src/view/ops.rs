use crate::db::ty::DatabaseValue;
use crate::err::RuntimeResult;
use crate::query::Expr;
use crate::view::{
    ComputationView, ComputationViewBox, ExprView, ExprViewBox, SingleExprView, Value, ValueCount,
    View, ViewBox,
};
use generic_array::sequence::Split;
use generic_array::typenum::operator_aliases::Sum;
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
                l: ExprViewBox<Self>,
                r: ExprViewBox<Rhs>,
            ) -> ExprViewBox<Self::Result>;
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
            LL: ValueCount + Add<RL, Output=OL>,
            RL: ValueCount,
            OL: ValueCount + Sub<LL, Output=RL>,
        > View<O, OL> for $computation_name<L, R, O, LL, RL> {
            fn eval(&self, v: &GenericArray<DatabaseValue, OL>) -> RuntimeResult<O> {
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
                LL: ValueCount + Add<RL, Output = OL>,
                RL: ValueCount,
                OL: ValueCount + Sub<LL, Output = RL>,
            > ComputationView<O, OL> for $computation_name<L, R, O, LL, RL>
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
            > $ops_trait<ExprViewBox<R>> for ExprViewBox<L>
        {
            type Output = ExprViewBox<O>;

            fn $ops_method(self, rhs: ExprViewBox<R>) -> Self::Output {
                L::$trait_method(self, rhs)
            }
        }

        impl<
                L: Value + $expr_trait<R, Result = O>,
                R: Value,
                O: Value,
            > $ops_trait<R> for ExprViewBox<L>
        {
            type Output = ExprViewBox<O>;

            fn $ops_method(self, rhs: R) -> Self::Output {
                L::$trait_method(self, rhs.view())
            }
        }

        impl<
                L: Value + $ops_trait<R, Output = O>,
                R: 'static,
                O: 'static,
                RL: ValueCount,
                OL: ValueCount + Sub<<L as Value>::L, Output = RL>
            > $ops_trait<ComputationViewBox<R, RL>> for ExprViewBox<L>
        where <L as Value>::L: Add<RL, Output = OL>
        {
            type Output = ComputationViewBox<O, OL>;

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
                LL: ValueCount + Add<<R as Value>::L, Output = OL>,
                OL: ValueCount + Sub<LL, Output = <R as Value>::L>
            > $ops_trait<R> for ComputationViewBox<L, LL>
        {
            type Output = ComputationViewBox<O, OL>;

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
                LL: ValueCount + Add<<R as Value>::L, Output = OL>,
                OL: ValueCount + Sub<LL, Output = <R as Value>::L>
            > $ops_trait<ExprViewBox<R>> for ComputationViewBox<L, LL>
        {
            type Output = ComputationViewBox<O, OL>;

            fn $ops_method(self, rhs: ExprViewBox<R>) -> Self::Output {
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
                LL: ValueCount + Add<RL, Output = OL>,
                RL: ValueCount,
                OL: ValueCount + Sub<LL, Output = RL>,
            > $ops_trait<ComputationViewBox<R, RL>> for ComputationViewBox<L, LL>
        {
            type Output = ComputationViewBox<O, OL>;

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

                    fn $trait_method(
                        l: ExprViewBox<Self>,
                        r: ExprViewBox<$ty>,
                    ) -> ExprViewBox<Self::Result> {
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
