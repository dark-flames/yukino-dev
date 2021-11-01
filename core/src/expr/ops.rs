use crate::expr::QueryResultNode;
use std::ops::Add;

pub trait ResultAdd<Rhs: 'static + Clone = Self>: 'static + Clone {
    type Output: 'static + Clone;

    fn add(l: &QueryResultNode<Self>, r: &QueryResultNode<Rhs>) -> QueryResultNode<Self::Output>
        where
            Self: Sized;
}

impl<L, R, O> Add<&QueryResultNode<R>> for &QueryResultNode<L>
    where
        L: ResultAdd<R, Output=O>,
        R: 'static + Clone,
        O: 'static + Clone,
{
    type Output = QueryResultNode<O>;

    fn add(self, r: &QueryResultNode<R>) -> Self::Output {
        L::add(self, r)
    }
}
