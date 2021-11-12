use crate::db::ty::DatabaseValue;
use crate::err::RuntimeResult;
use crate::query::Expr;
use crate::view::{
    ComputationView, ComputationViewBox, ExprView, ExprViewBox, SingleExprView, Value, ValueCount,
    View, ViewBox,
};
use generic_array::sequence::Split;
use generic_array::typenum::Sum;
use generic_array::{arr, GenericArray};
use std::ops::{Add, Div, Mul, Sub};

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
        pub trait $expr_trait<Rhs: Value<L = Self::RL>>:
            Value<L = Self::LL> + $ops_trait<Rhs, Output = Self::Result>
        {
            type LL: ValueCount;
            type RL: ValueCount;
            type OL: ValueCount;
            type Result: Value<L = Self::OL>;

            fn $trait_method(
                l: ExprViewBox<Self, Self::LL>,
                r: ExprViewBox<Rhs, Self::RL>,
            ) -> ExprViewBox<Self::Result, Self::OL>;
        }

        pub struct $computation_name<
            L: Value<L = LL> + $ops_trait<R, Output = O>,
            R: Value<L = RL>,
            O: 'static,
            LL: ValueCount + Add<RL>,
            RL: ValueCount,
        > {
            l: ViewBox<L, LL>,
            r: ViewBox<R, RL>,
        }

        impl<
                L: Value<L = LL> + $ops_trait<R, Output = O>,
                R: Value<L = RL>,
                O: 'static,
                LL: ValueCount + Add<RL, Output = OL>,
                RL: ValueCount,
                OL: ValueCount + Sub<LL, Output = RL>,
            > View<O, Sum<LL, RL>> for $computation_name<L, R, O, LL, RL>
        {
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
                L: Value<L = LL> + $ops_trait<R, Output = O>,
                R: Value<L = RL>,
                O: 'static,
                LL: ValueCount + Add<RL, Output = OL>,
                RL: ValueCount,
                OL: ValueCount + Sub<LL, Output = RL>,
            > ComputationView<O, Sum<LL, RL>> for $computation_name<L, R, O, LL, RL>
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
                L: Value<L = LL> + $expr_trait<R, LL = LL, RL = RL, OL = OL, Result = O>,
                R: Value<L = RL>,
                O: Value<L = OL>,
                LL: ValueCount,
                RL: ValueCount,
                OL: ValueCount,
            > $ops_trait<ExprViewBox<R, RL>> for ExprViewBox<L, LL>
        {
            type Output = ExprViewBox<O, OL>;

            fn $ops_method(self, rhs: ExprViewBox<R, RL>) -> Self::Output {
                L::$trait_method(self, rhs)
            }
        }

        impl<
                L: Value<L = LL> + $expr_trait<R, LL = LL, RL = RL, OL = OL, Result = O>,
                R: Value<L = RL>,
                O: Value<L = OL>,
                LL: ValueCount,
                RL: ValueCount,
                OL: ValueCount,
            > $ops_trait<R> for ExprViewBox<L, LL>
        {
            type Output = ExprViewBox<O, OL>;

            fn $ops_method(self, rhs: R) -> Self::Output {
                L::$trait_method(self, rhs.view())
            }
        }

        impl<
                L: Value<L = LL> + $expr_trait<R, LL = LL, RL = RL, OL = OL, Result = O>,
                R: Value<L = RL>,
                O: Value<L = OL>,
                LL: ValueCount + Add<RL, Output = OL>,
                RL: ValueCount,
                OL: ValueCount + Sub<LL, Output = RL>,
            > $ops_trait<ComputationViewBox<R, RL>> for ExprViewBox<L, LL>
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
                L: Value<L = LL> + $expr_trait<R, LL = LL, RL = RL, OL = OL, Result = O>,
                R: Value<L = RL>,
                O: Value<L = OL>,
                LL: ValueCount + Add<RL, Output = OL>,
                RL: ValueCount,
                OL: ValueCount + Sub<LL, Output = RL>,
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
                L: Value<L = LL> + $expr_trait<R, LL = LL, RL = RL, OL = OL, Result = O>,
                R: Value<L = RL>,
                O: Value<L = OL>,
                LL: ValueCount + Add<RL, Output = OL>,
                RL: ValueCount,
                OL: ValueCount + Sub<LL, Output = RL>,
            > $ops_trait<ExprViewBox<R, RL>> for ComputationViewBox<L, LL>
        {
            type Output = ComputationViewBox<O, OL>;

            fn $ops_method(self, rhs: ExprViewBox<R, RL>) -> Self::Output {
                Box::new($computation_name {
                    l: self.view_clone(),
                    r: rhs.view_clone(),
                })
            }
        }

        impl<
                L: Value<L = LL> + $expr_trait<R, LL = LL, RL = RL, OL = OL, Result = O>,
                R: Value<L = RL>,
                O: Value<L = OL>,
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
                    type LL = <$ty as Value>::L;
                    type RL = <$ty as Value>::L;
                    type OL = <$ty as Value>::L;
                    type Result = $ty;

                    fn $trait_method(
                        l: ExprViewBox<Self, Self::LL>,
                        r: ExprViewBox<$ty, Self::RL>,
                    ) -> ExprViewBox<Self::Result, Self::OL> {
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

impl_ops!(+, ExprAdd, expr_add, AddView, Add, add, Add, impl_add, [u16, i16, u32, i32, u64, i64, f32, f64]);
impl_ops!(-, ExprSub, expr_sub, SubView, Sub, sub, Sub, impl_sub, [u16, i16, u32, i32, u64, i64, f32, f64]);
impl_ops!(*, ExprMul, expr_mul, MulView, Mul, mul, Mul, impl_mul, [u16, i16, u32, i32, u64, i64, f32, f64]);
impl_ops!(/, ExprDiv, expr_div, DivView, Div, div, Div, impl_div, [u16, i16, u32, i32, u64, i64, f32, f64]);
