mod basic;

use crate::query::computation::Computation;
use crate::query::optimizer::QueryOptimizer;
pub use basic::*;

pub trait View {
    type Output: Clone;
    fn computation<'f>(&self) -> Computation<'f, Self::Output>;

    fn optimizer(&self) -> Box<dyn QueryOptimizer>;
}

pub type ViewBox<V> = Box<dyn View<Output = V>>;
