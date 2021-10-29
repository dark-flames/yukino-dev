use crate::query::computation::Computation;
use crate::query::optimizer::{OptimizerCombinator, QueryOptimizer};
use crate::view::{View, ViewBox};
use std::ops::Add;

pub struct AddView<V: Clone + Add<V, Output=V>> {
    left: ViewBox<V>,
    right: ViewBox<V>,
}

impl<V: Clone + Add<V, Output=V>> View for AddView<V> {
    type Output = V;

    fn computation<'f>(&self) -> Computation<'f, Self::Output> {
        let computation_l = self.left.computation();
        let computation_r = self.right.computation();

        Computation::create(Box::new(move |v| {
            Ok(computation_l.eval(v)? + computation_r.eval(v)?)
        }))
    }

    fn optimizer(&self) -> Box<dyn QueryOptimizer> {
        // todo: push calculate
        OptimizerCombinator::create(self.left.optimizer(), self.right.optimizer())
    }
}

impl<V: Clone + Add<V, Output=V>> Add<Box<dyn View<Output=V>>> for Box<dyn View<Output=V>> {
    type Output = AddView<V>;

    fn add(self, rhs: Box<dyn View<Output=V>>) -> Self::Output {
        AddView {
            left: self,
            right: rhs,
        }
    }
}
