use crate::db::ty::DatabaseValue;
use crate::err::RuntimeResult;
use crate::query::Expr;
use crate::view::value::Value;
use crate::view::{Computation, ComputationView, ConstView, ExprView, View, ViewBox, ViewNode};

macro_rules! expr_binary_ops {
    ($op: ident, $node: ident, $trait: ident, $method: ident, $operator: tt, $macro_name: ident) => {
        use std::ops::$op;
        #[derive(Clone, Debug)]
        pub struct $node<L, R, O>
        where
            L: Value + $op<R, Output = O>,
            R: Value,
            O: Value,
        {
            l: ViewNode<L>,
            r: ViewNode<R>,
        }

        impl<L, R, O> Computation for $node<L, R, O>
        where
            L: Value + $op<R, Output = O>,
            R: Value,
            O: Value,
        {
            type Output = O;

            fn eval(&self, v: &[&DatabaseValue]) -> RuntimeResult<Self::Output> {
                let mid = L::converter().param_count();
                Ok(self.l.eval(&v[0..mid])? $operator self.r.eval(&v[mid..O::converter().param_count()])?)
            }
        }

        impl<L, R, O> View<O> for $node<L, R, O>
        where
            L: Value + $op<R, Output = O>,
            R: Value,
            O: Value,
        {
            fn view_node(&self) -> ViewNode<O> {
                ViewNode::Computation(Box::new(Clone::clone(self)))
            }

            fn collect_expr(&self) -> Vec<Expr> {
                let mut exprs = self.l.collect_expr();

                exprs.extend(self.r.collect_expr());

                exprs
            }

            fn clone(&self) -> ViewBox<O> {
                Box::new(Clone::clone(self))
            }
        }

        impl<L, R, O> ComputationView<O> for $node<L, R, O>
        where
            L: Value + $op<R, Output = O>,
            R: Value,
            O: Value,
        {
            fn computation_view_box_clone(&self) -> Box<dyn ComputationView<O>> {
                Box::new(Clone::clone(self))
            }
        }

        pub trait $trait<Rhs: Value = Self>: Value + $op<Rhs, Output = Self::Result> {
            type Result: Value;

            fn $method(l: ExprView<Self>, r: ExprView<Rhs>) -> ExprView<<Self as $trait<Rhs>>::Result>
            where
                Self: Sized;
        }

        impl<L, R, O> $op<ViewBox<R>> for ViewBox<L>
        where
            L: Value + $trait<R, Result = O>,
            R: Value,
            O: Value,
        {
            type Output = ViewBox<O>;

            fn $method(self, rhs: Box<dyn View<R>>) -> Self::Output {
                Box::new(match (self.view_node(), rhs.view_node()) {
                    (ViewNode::Expr(l), ViewNode::Expr(r)) => ViewNode::Expr(<L as $trait<R>>::$method(l, r)),
                    (ViewNode::Expr(l), ViewNode::Const(r)) => {
                        ViewNode::Expr(<L as $trait<R>>::$method(l, r.try_into().unwrap()))
                    }
                    (ViewNode::Const(l), ViewNode::Expr(r)) => {
                        ViewNode::Expr(<L as $trait<R>>::$method(l.try_into().unwrap(), r))
                    }
                    (ViewNode::Const(l), ViewNode::Const(r)) => {
                        ViewNode::Const(ConstView::create(l.value $operator r.value))
                    }
                    (l, r) => ViewNode::Computation(Box::new($node { l, r })),
                })
            }
        }

        impl<L, R, O> $op<R> for ViewBox<L>
        where
            L: Value + $trait<R, Result = O>,
            R: Value,
            O: Value,
        {
            type Output = ViewBox<O>;

            fn $method(self, rhs: R) -> Self::Output {
                let r: ViewBox<R> = Box::new(ViewNode::Const(ConstView::create(rhs)));
                self $operator r
            }
        }

        macro_rules! $macro_name {
            ($ty: ty) => {
                impl $trait<$ty> for $ty {
                    type Result = $ty;

                    fn $method(l: ExprView<Self>, r: ExprView<$ty>) -> ExprView<<Self as $trait<$ty>>::Result>
                    where
                        Self: Sized {

                        ExprView::create(l.exprs.into_iter().zip(r.exprs.into_iter()).map(
                            |(l_i, r_i)| Expr::$op(Box::new(l_i), Box::new(r_i))
                        ).collect())
                    }
                }
            }
        }

        $macro_name!(u16);
        $macro_name!(i16);
        $macro_name!(u32);
        $macro_name!(i32);
        $macro_name!(u64);
        $macro_name!(i64);
        $macro_name!(f32);
        $macro_name!(f64);
    }
}

expr_binary_ops!(Add, AddComputationNode, ExprAdd, add, +, impl_add_basic);
expr_binary_ops!(Sub, SubComputationNode, ExprSub, sub, -, impl_sub_basic);
expr_binary_ops!(Mul, MulComputationNode, ExprMul, mul, *, impl_mul_basic);
expr_binary_ops!(Div, DivComputationNode, ExprDiv, div, /, impl_div_basic);
