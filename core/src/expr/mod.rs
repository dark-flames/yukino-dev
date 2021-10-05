use crate::query::calc::Computation;

pub trait Expr<V: Clone> {
    fn computation<'f>(&self) -> Computation<'f, V>;
}