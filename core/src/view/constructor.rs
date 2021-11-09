use crate::db::ty::DatabaseValue;
use crate::err::RuntimeResult;
use crate::query::Expr;
use crate::view::{ComputationView, ExprView, Value, View, ViewNode};
use std::marker::PhantomData;

pub trait ViewCombinator<Unwrapped> {
    fn unwrap(self) -> Unwrapped;
}

#[derive(Debug)]
pub struct TupleView<LView, RView, L, R>
    where
        L: Value,
        R: Value,
        LView: View<L>,
        RView: View<R>,
{
    l: Box<LView>,
    r: Box<RView>,
    _l_marker: PhantomData<L>,
    _r_marker: PhantomData<R>,
}

impl<LView, RView, L, R> View<(L, R)> for TupleView<LView, RView, L, R>
    where
        L: Value,
        R: Value,
        LView: View<L>,
        RView: View<R>,
{
    fn view_node(&self) -> ViewNode<(L, R)> {
        match (self.l.view_node(), self.r.view_node()) {
            (ViewNode::Expr(l_expr), ViewNode::Expr(r_expr)) => ViewNode::Expr(ExprView::create(
                vec![l_expr.exprs, r_expr.exprs]
                    .into_iter()
                    .flatten()
                    .collect(),
            )),
            _ => ViewNode::Computation(self.computation_view_box_clone()),
        }
    }

    fn collect_expr(&self) -> Vec<Expr> {
        self.view_node().collect_expr()
    }

    fn param_count(&self) -> usize {
        self.l.param_count() + self.param_count()
    }

    fn eval(&self, v: &[&DatabaseValue]) -> RuntimeResult<(L, R)> {
        let l_count = self.l.param_count();
        let r_count = self.r.param_count();

        Ok((
            self.l.eval(&v[0..l_count])?,
            self.r.eval(&v[l_count..=r_count])?,
        ))
    }

    fn clone(&self) -> Self
        where
            Self: Sized,
    {
        Clone::clone(self)
    }
}

impl<LView, RView, L, R> ComputationView<(L, R)> for TupleView<LView, RView, L, R>
    where
        L: Value,
        R: Value,
        LView: View<L>,
        RView: View<R>,
{
    fn computation_view_box_clone(&self) -> Box<dyn ComputationView<(L, R)>> {
        Box::new(Clone::clone(self))
    }
}

impl<LView, RView, L, R> Clone for TupleView<LView, RView, L, R>
    where
        L: Value,
        R: Value,
        LView: View<L>,
        RView: View<R>,
{
    fn clone(&self) -> Self {
        TupleView {
            l: Box::new(self.l.clone()),
            r: Box::new(self.r.clone()),
            _l_marker: Default::default(),
            _r_marker: Default::default(),
        }
    }
}

impl<LView, RView, L, R> ViewCombinator<(LView, RView)> for TupleView<LView, RView, L, R>
    where
        L: Value,
        R: Value,
        LView: View<L>,
        RView: View<R>,
{
    fn unwrap(self) -> (LView, RView) {
        (*self.l, *self.r)
    }
}
