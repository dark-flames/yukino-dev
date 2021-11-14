use crate::db::ty::DatabaseValue;
use crate::err::RuntimeResult;
use crate::query::Expr;
use crate::view::{
    ComputationView, ComputationViewBox, ExprView, ExprViewBox, SingleExprView, Value, ValueCount,
    View, ViewBox,
};
use generic_array::sequence::Split;
use generic_array::{arr, GenericArray};
use std::ops::{Add, Sub};

macro_rules! op_trait {
    (
        $op_trait: ident,
        $op_trait_method: ident
    ) => {
        pub trait $op_trait<Rhs = Self> {
            fn $op_trait_method(&self, rhs: &Rhs) -> bool;
        }
    };
}
macro_rules! impl_op_for {
    (
        $op: tt,
        $op_trait: ident,
        $op_trait_method: ident,
        [$($ty: ty),*]
    ) => {
        $(
        impl $op_trait for $ty {
            fn $op_trait_method(&self, rhs: &Self) -> bool {
                self $op rhs
            }
        }
        )*
    }
}
macro_rules! impl_expr_for {
    (
        $expr_op_trait: ident,
        $expr_op_method: ident,
        $expr_variant: ident,
        [$($ty: ty),*]
    ) => {
        $(
        impl $expr_op_trait for $ty {
            fn $expr_op_method(l: ExprViewBox<Self>, r: ExprViewBox<Self>) -> ExprViewBox<bool> {
                let l_expr = l.collect_expr().into_iter().next().unwrap();
                let r_expr = r.collect_expr().into_iter().next().unwrap();
                let result = Expr::$expr_variant(Box::new(l_expr), Box::new(r_expr));
                Box::new(SingleExprView::from_exprs(arr![Expr; result]))
            }
        }
        )*
    }
}
macro_rules! impl_bool_operator {
    (
        $op: tt,
        $op_trait: ident,
        $op_trait_method: ident,
        $view_op_trait: ident,
        $view_op_method: ident,
        $expr_op_trait: ident,
        $expr_op_method: ident,
        $computation: ident,
        $expr_variant: ident,
        [$($ty: ty),*]
    ) => {
        pub trait $view_op_trait<Rhs = Self> {
            type Output;
            fn $view_op_method(self, rhs: Rhs) -> Self::Output;
        }


        pub trait $expr_op_trait<Rhs: Value = Self>: Value + $op_trait<Rhs> {
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
                let r_result = self.r.eval(r)?;
                Ok(self.l.eval(l)?.$op_trait_method(&r_result))
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

        impl_expr_for! (
            $expr_op_trait,
            $expr_op_method,
            $expr_variant,
            [$($ty),*]
        );
    };
}

macro_rules! generate_macro {
    ($name: ident, $view_trait: ident, $view_trait_method: ident) => {
        #[macro_export]
        macro_rules! $name {
            ($l: expr, $r: expr) => {{
                use yukino::operator::$view_trait;
                ($l).$view_trait_method($r)
            }};
        }
    };
}

op_trait!(And, and);
op_trait!(Or, or);
op_trait!(Bt, bt);
op_trait!(Bte, bte);
op_trait!(Lt, lt);
op_trait!(Lte, lte);

impl_op_for!(&, And, and, [bool]);
impl_op_for!(|, Or, or, [bool]);
impl_op_for!(>, Bt, bt, [bool, u16, i16, u32, i32, u64, i64, f32, f64, char, String]);
impl_op_for!(>=, Bte, bte, [bool, u16, i16, u32, i32, u64, i64, f32, f64, char, String]);
impl_op_for!(<, Lt, lt, [bool, u16, i16, u32, i32, u64, i64, f32, f64, char, String]);
impl_op_for!(<=, Lte, lte, [bool, u16, i16, u32, i32, u64, i64, f32, f64, char, String]);

impl_bool_operator!(&, And, and, ViewAnd, view_and, ExprAnd, expr_and, AndView, And, [bool]);
impl_bool_operator!(|, Or, or, ViewOr, view_or, ExprOr, expr_or, OrView, Or, [bool]);
impl_bool_operator!(==, PartialEq, eq, ViewEq, view_eq, ExprEq, expr_eq, EqView, Eq,
    [bool, u16, i16, u32, i32, u64, i64, f32, f64, char, String]);
impl_bool_operator!(!=, PartialEq, ne, ViewNeq, view_neq, ExprNeq, expr_neq, NeqView, Neq,
    [bool, u16, i16, u32, i32, u64, i64, f32, f64, char, String]);
impl_bool_operator!(>, Bt, bt, ViewBt, view_bt, ExprBt, expr_bt, BtView, Bt,
    [u16, i16, u32, i32, u64, i64, f32, f64, char, String]);
impl_bool_operator!(>=, Bte, bte, ViewBte, view_bte, ExprBte, expr_bte, BteView, Bte,
    [u16, i16, u32, i32, u64, i64, f32, f64, char, String]);
impl_bool_operator!(<, Lt, lt, ViewLt, view_lt, ExprLt, expr_lt, LtView, Lt,
    [u16, i16, u32, i32, u64, i64, f32, f64, char, String]);
impl_bool_operator!(<=, Lte, lte, ViewLte, view_lte, ExprLte, expr_lte, LteView, Lte,
    [u16, i16, u32, i32, u64, i64, f32, f64, char, String]);

generate_macro!(and, ViewAnd, view_and);
generate_macro!(or, ViewOr, view_or);
generate_macro!(eq, ViewEq, view_eq);
generate_macro!(neq, ViewNeq, view_neq);
generate_macro!(bt, ViewBt, view_bt);
generate_macro!(bte, ViewBte, view_bte);
generate_macro!(lt, ViewLt, view_lt);
generate_macro!(lte, ViewLte, view_lte);
