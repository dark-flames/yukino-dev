use crate::db::ty::DatabaseValue;
use crate::err::RuntimeResult;
use crate::query::Expr;
use crate::view::value::Value;
use crate::view::{Computation, ComputationView, ConstView, ExprView, View, ViewBox, ViewNode};
use std::ops::Add;

#[derive(Clone)]
pub struct AddComputationNode<L, R, O>
    where
        L: Value + Add<R, Output=O>,
        R: Value,
        O: Value,
{
    l: ViewNode<L>,
    r: ViewNode<R>,
}

impl<L, R, O> Computation for AddComputationNode<L, R, O>
    where
        L: Value + Add<R, Output=O>,
        R: Value,
        O: Value,
{
    type Output = O;

    fn eval(&self, v: &[&DatabaseValue]) -> RuntimeResult<Self::Output> {
        let mid = L::converter().param_count();
        Ok(self.l.eval(&v[0..mid])? + self.r.eval(&v[mid..O::converter().param_count()])?)
    }
}

impl<L, R, O> View<O> for AddComputationNode<L, R, O>
    where
        L: Value + Add<R, Output=O>,
        R: Value,
        O: Value,
{
    fn view_node(&self) -> ViewNode<O> {
        ViewNode::Computation(Box::new(self.clone()))
    }

    fn collect_expr(&self) -> Vec<Expr> {
        let mut exprs = self.l.collect_expr();

        exprs.extend(self.r.collect_expr());

        exprs
    }

    fn box_clone(&self) -> ViewBox<O> {
        Box::new(self.clone())
    }
}

impl<L, R, O> ComputationView<O> for AddComputationNode<L, R, O>
    where
        L: Value + Add<R, Output=O>,
        R: Value,
        O: Value,
{
    fn computation_view_box_clone(&self) -> Box<dyn ComputationView<O>> {
        Box::new(self.clone())
    }
}

pub trait ExprAdd<Rhs: Value = Self>: Value + Add<Rhs, Output=Self::Result> {
    type Result: Value;

    fn add(l: ExprView<Self>, r: ExprView<Rhs>) -> ExprView<<Self as ExprAdd<Rhs>>::Result>
        where
            Self: Sized;
}

impl<L, R, O> Add<ViewBox<R>> for ViewBox<L>
    where
        L: Value + ExprAdd<R, Result=O>,
        R: Value,
        O: Value,
{
    type Output = ViewBox<O>;

    fn add(self, rhs: Box<dyn View<R>>) -> Self::Output {
        Box::new(match (self.view_node(), rhs.view_node()) {
            (ViewNode::Expr(l), ViewNode::Expr(r)) => ViewNode::Expr(<L as ExprAdd<R>>::add(l, r)),
            (ViewNode::Expr(l), ViewNode::Const(r)) => {
                ViewNode::Expr(<L as ExprAdd<R>>::add(l, r.try_into().unwrap()))
            }
            (ViewNode::Const(l), ViewNode::Expr(r)) => {
                ViewNode::Expr(<L as ExprAdd<R>>::add(l.try_into().unwrap(), r))
            }
            (ViewNode::Const(l), ViewNode::Const(r)) => {
                ViewNode::Const(ConstView::create(l.value + r.value))
            }
            (l, r) => ViewNode::Computation(Box::new(AddComputationNode { l, r })),
        })
    }
}

impl<L, R, O> Add<R> for ViewBox<L>
    where
        L: Value + ExprAdd<R, Result=O>,
        R: Value,
        O: Value,
{
    type Output = ViewBox<O>;

    fn add(self, rhs: R) -> Self::Output {
        let r: ViewBox<R> = Box::new(ViewNode::Const(ConstView::create(rhs)));
        self + r
    }
}
