use crate::db::ty::DatabaseValue;
use crate::err::{RuntimeError, RuntimeResult};
use crate::query::Expr;
use crate::view::{Computation, Value};
use std::marker::PhantomData;

pub type ViewBox<T> = Box<dyn View<T>>;

pub enum ViewNode<T: Value> {
    Expr(ExprView<T>),
    Const(ConstView<T>),
    Computation(Box<dyn ComputationView<T>>),
}

#[derive(Clone)]
pub struct ExprView<T: Value> {
    pub exprs: Vec<Expr>,
    _marker: PhantomData<T>,
}

#[derive(Clone)]
pub struct ConstView<T: Value> {
    pub value: T,
}

pub trait ComputationView<T: Value>: Computation<Output=T> + View<T> {
    fn computation_view_box_clone(&self) -> Box<dyn ComputationView<T>>;
}

pub trait View<T: Value>: Computation<Output=T> {
    fn view_node(&self) -> ViewNode<T>;

    fn collect_expr(&self) -> Vec<Expr>;

    fn box_clone(&self) -> ViewBox<T>;
}

impl<T: Value> TryFrom<ConstView<T>> for ExprView<T> {
    type Error = RuntimeError;

    fn try_from(c: ConstView<T>) -> Result<Self, Self::Error> {
        Ok(ExprView::create(
            T::converter()
                .serialize(&c.value)?
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
            ViewNode::Expr(n) => ViewNode::Expr(n.clone()),
            ViewNode::Const(n) => ViewNode::Const(n.clone()),
            ViewNode::Computation(n) => ViewNode::Computation(n.computation_view_box_clone()),
        }
    }
}

impl<T: Value> Computation for ExprView<T> {
    type Output = T;

    fn eval(&self, v: &[&DatabaseValue]) -> RuntimeResult<Self::Output> {
        (*T::converter().deserializer())(v)
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
        ViewNode::Expr(self.clone())
    }

    fn collect_expr(&self) -> Vec<Expr> {
        self.exprs.clone()
    }

    fn box_clone(&self) -> ViewBox<T> {
        Box::new(self.clone())
    }
}

impl<T: Value> View<T> for ConstView<T> {
    fn view_node(&self) -> ViewNode<T> {
        ViewNode::Const(self.clone())
    }

    fn collect_expr(&self) -> Vec<Expr> {
        vec![]
    }

    fn box_clone(&self) -> ViewBox<T> {
        Box::new(self.clone())
    }
}

impl<T: Value> View<T> for ViewNode<T> {
    fn view_node(&self) -> ViewNode<T> {
        self.clone()
    }

    fn collect_expr(&self) -> Vec<Expr> {
        match self {
            ViewNode::Expr(e) => e.collect_expr(),
            ViewNode::Const(c) => c.collect_expr(),
            ViewNode::Computation(c) => c.collect_expr(),
        }
    }

    fn box_clone(&self) -> ViewBox<T> {
        match self {
            ViewNode::Expr(e) => Box::new(ViewNode::Expr(e.clone())),
            ViewNode::Const(c) => Box::new(ViewNode::Const(c.clone())),
            ViewNode::Computation(c) => {
                Box::new(ViewNode::Computation(c.computation_view_box_clone()))
            }
        }
    }
}
