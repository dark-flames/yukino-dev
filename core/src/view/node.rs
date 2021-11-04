use crate::db::ty::DatabaseValue;
use crate::err::{ErrorOnView, RuntimeResult, ViewError, YukinoError};
use crate::query::Expr;
use crate::view::{Computation, Value};
use std::fmt::Debug;
use std::marker::PhantomData;

pub type ViewBox<T> = Box<dyn View<T>>;

#[derive(Debug)]
pub enum ViewNode<T: Value> {
    Expr(ExprView<T>),
    Const(ConstView<T>),
    Computation(Box<dyn ComputationView<T>>),
}

#[derive(Clone, Debug)]
pub struct ExprView<T: Value> {
    pub exprs: Vec<Expr>,
    _marker: PhantomData<T>,
}

#[derive(Clone, Debug)]
pub struct ConstView<T: Value> {
    pub value: T,
}

pub trait ComputationView<T: Value>: Computation<Output=T> + View<T> + Debug {
    fn computation_view_box_clone(&self) -> Box<dyn ComputationView<T>>;
}

pub trait View<T: Value>: Computation<Output=T> + Debug {
    fn view_node(&self) -> ViewNode<T>;

    fn collect_expr(&self) -> Vec<Expr>;

    fn clone(&self) -> ViewBox<T>;
}

impl<T: Value> TryFrom<ConstView<T>> for ExprView<T> {
    type Error = ViewError;

    fn try_from(c: ConstView<T>) -> Result<Self, Self::Error> {
        Ok(ExprView::create(
            T::converter()
                .serialize(&c.value)
                .map_err(|e| e.as_view_err(&c))?
                .into_iter()
                .map(Expr::Lit)
                .collect(),
        ))
    }
}

impl<T: Value> ConstView<T> {
    pub fn create(value: T) -> Self {
        ConstView { value }
    }
}

impl<T: Value> ExprView<T> {
    pub fn create(exprs: Vec<Expr>) -> Self {
        assert_eq!(T::converter().param_count(), exprs.len());
        ExprView {
            exprs,
            _marker: Default::default(),
        }
    }
}

impl<T: Value> Clone for ViewNode<T> {
    fn clone(&self) -> Self {
        match self {
            ViewNode::Expr(n) => ViewNode::Expr(Clone::clone(n)),
            ViewNode::Const(n) => ViewNode::Const(Clone::clone(n)),
            ViewNode::Computation(n) => ViewNode::Computation(n.computation_view_box_clone()),
        }
    }
}

impl<T: Value> Computation for ExprView<T> {
    type Output = T;

    fn eval(&self, v: &[&DatabaseValue]) -> RuntimeResult<Self::Output> {
        (*T::converter().deserializer())(v).map_err(|e| e.as_runtime_err())
    }
}

impl<T: Value> Computation for ConstView<T> {
    type Output = T;

    fn eval(&self, _v: &[&DatabaseValue]) -> RuntimeResult<Self::Output> {
        Ok(self.value.clone())
    }
}

impl<T: Value> Computation for ViewNode<T> {
    type Output = T;

    fn eval(&self, v: &[&DatabaseValue]) -> RuntimeResult<Self::Output> {
        match self {
            ViewNode::Expr(expr) => expr.eval(v),
            ViewNode::Const(constant) => constant.eval(v),
            ViewNode::Computation(computation) => computation.eval(v),
        }
    }
}

impl<T: Value> View<T> for ExprView<T> {
    fn view_node(&self) -> ViewNode<T> {
        ViewNode::Expr(Clone::clone(self))
    }

    fn collect_expr(&self) -> Vec<Expr> {
        self.exprs.clone()
    }

    fn clone(&self) -> ViewBox<T> {
        Box::new(Clone::clone(self))
    }
}

impl<T: Value> View<T> for ConstView<T> {
    fn view_node(&self) -> ViewNode<T> {
        ViewNode::Const(Clone::clone(self))
    }

    fn collect_expr(&self) -> Vec<Expr> {
        vec![]
    }

    fn clone(&self) -> ViewBox<T> {
        Box::new(Clone::clone(self))
    }
}

impl<T: Value> View<T> for ViewNode<T> {
    fn view_node(&self) -> ViewNode<T> {
        Clone::clone(self)
    }

    fn collect_expr(&self) -> Vec<Expr> {
        match self {
            ViewNode::Expr(e) => e.collect_expr(),
            ViewNode::Const(c) => c.collect_expr(),
            ViewNode::Computation(c) => c.collect_expr(),
        }
    }

    fn clone(&self) -> ViewBox<T> {
        match self {
            ViewNode::Expr(e) => Box::new(ViewNode::Expr(Clone::clone(e))),
            ViewNode::Const(c) => Box::new(ViewNode::Const(Clone::clone(c))),
            ViewNode::Computation(c) => {
                Box::new(ViewNode::Computation(c.computation_view_box_clone()))
            }
        }
    }
}
