use crate::db::ty::DatabaseValue;
use crate::err::RuntimeResult;
use crate::view::{
    ComputationView, ComputationViewBox, ExprViewBox, Value, ValueCount, View, ViewBox,
};
use generic_array::sequence::Split;
use generic_array::GenericArray;
use std::ops::{Add, Sub};

macro_rules! impl_bool_operator {
    (
        $op_trait: ident,
        $op_trait_method: ident,
        $view_op_trait: ident,
        $view_op_method: ident,
        $expr_op_trait: ident,
        $expr_op_method: ident,
        $computation: ident
    ) => {
        pub trait $op_trait<Rhs = Self> {
            fn $op_trait_method(self, rhs: Rhs) -> bool;
        }

        pub trait $view_op_trait<Rhs = Self> {
            type Output;
            fn $view_op_method(self, rhs: Rhs) -> Self::Output;
        }

        pub trait $expr_op_trait<Rhs: Value>: Value + $op_trait<Rhs> {
            fn $expr_op_method(l: ExprViewBox<Self>, r: ExprViewBox<Rhs>) -> ExprViewBox<bool>;
        }

        pub struct $computation<
            L: 'static + $op_trait<R>,
            R: 'static,
            LL: ValueCount,
            RL: ValueCount,
        > {
            l: ViewBox<L, LL>,
            r: ViewBox<R, RL>,
        }

        impl<
                L: 'static + $op_trait<R>,
                R: 'static,
                LL: ValueCount + Add<RL, Output = OL>,
                RL: ValueCount,
                OL: ValueCount + Sub<LL, Output = RL>,
            > View<bool, OL> for $computation<L, R, LL, RL>
        {
            fn eval(&self, v: &GenericArray<DatabaseValue, OL>) -> RuntimeResult<bool> {
                let (l, r) = Split::<_, LL>::split(v);
                Ok(self.l.eval(l)?.$op_trait_method(self.r.eval(r)?))
            }

            fn view_clone(&self) -> ViewBox<bool, OL> {
                Box::new($computation {
                    l: self.l.view_clone(),
                    r: self.r.view_clone(),
                })
            }
        }

        impl<
                L: 'static + $op_trait<R>,
                R: 'static,
                LL: ValueCount + Add<RL, Output = OL>,
                RL: ValueCount,
                OL: ValueCount + Sub<LL, Output = RL>,
            > ComputationView<bool, OL> for $computation<L, R, LL, RL>
        {
            fn computation_clone(&self) -> ComputationViewBox<bool, OL> {
                Box::new($computation {
                    l: self.l.view_clone(),
                    r: self.r.view_clone(),
                })
            }
        }

        impl<L: Value + $expr_op_trait<R>, R: Value> $view_op_trait<ExprViewBox<R>>
            for ExprViewBox<L>
        {
            type Output = ExprViewBox<bool>;

            fn $view_op_method(self, rhs: ExprViewBox<R>) -> Self::Output {
                L::$expr_op_method(self, rhs)
            }
        }

        impl<
                L: Value<L = LL> + $op_trait<R>,
                R: 'static,
                LL: ValueCount + Add<RL, Output = OL>,
                RL: ValueCount,
                OL: ValueCount + Sub<LL, Output = RL>,
            > $view_op_trait<ComputationViewBox<R, RL>> for ExprViewBox<L>
        {
            type Output = ComputationViewBox<bool, OL>;

            fn $view_op_method(self, rhs: ComputationViewBox<R, RL>) -> Self::Output {
                Box::new($computation {
                    l: self.view_clone(),
                    r: rhs.view_clone(),
                })
            }
        }

        impl<L: Value + $expr_op_trait<R>, R: Value> $view_op_trait<R> for ExprViewBox<L> {
            type Output = ExprViewBox<bool>;

            fn $view_op_method(self, rhs: R) -> Self::Output {
                L::$expr_op_method(self, rhs.view())
            }
        }

        impl<
                L: 'static + $op_trait<R>,
                R: Value,
                LL: ValueCount + Add<<R as Value>::L, Output = OL>,
                OL: ValueCount + Sub<LL, Output = <R as Value>::L>,
            > $view_op_trait<ExprViewBox<R>> for ComputationViewBox<L, LL>
        {
            type Output = ComputationViewBox<bool, OL>;

            fn $view_op_method(self, rhs: ExprViewBox<R>) -> Self::Output {
                Box::new($computation {
                    l: self.view_clone(),
                    r: rhs.view_clone(),
                })
            }
        }
    };
}

impl_bool_operator!(And, and, ViewAnd, view_and, ExprAnd, expr_and, AndView);
impl_bool_operator!(Or, or, ViewOr, view_or, ExprOr, expr_or, OrView);
impl_bool_operator!(Eq, eq, ViewEq, view_eq, ExprEq, expr_eq, EqView);
impl_bool_operator!(Neq, neq, ViewNeq, view_neq, ExprNeq, expr_neq, NeqView);
impl_bool_operator!(Bt, bt, ViewBt, view_bt, ExprBt, expr_bt, BtView);
impl_bool_operator!(Bte, bte, ViewBte, view_bte, ExprBte, expr_bte, BteView);
impl_bool_operator!(Lt, lt, ViewLt, view_lt, ExprLt, expr_lt, LtView);
impl_bool_operator!(Lte, lte, ViewLte, view_lte, ExprLte, expr_lte, LteView);
