use crate::query::calc::Computation;
use crate::query::optimizer::QueryOptimizer;

pub trait Expr {
    type Output: Clone;
    fn computation<'f>(&self) -> Computation<'f, Self::Output>;

    fn optimizer(&self) -> Box<dyn QueryOptimizer>;
}
