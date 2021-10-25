use std::marker::PhantomData;
use crate::entity::{Entity, EntityView};
use crate::err::RuntimeResult;
use crate::expr::{View, ViewBox};
use crate::query::query::Query;

pub struct QueryResult<E: Entity, V: Clone> {
    query: Query,
    current_view: ViewBox<V>,
    _marker: PhantomData<E>,
}

impl<E: Entity, V: Clone> QueryResult<E, V> {
    pub fn eval(self) -> RuntimeResult<V> {
        let result = self.query.execute();
        self.current_view.computation().eval(&result)
    }

    pub fn filter<F>(&mut self, f: F) -> &mut Self
        where
            F: Fn(E::View) -> Box<dyn View<Output=V>>,
    {
        let optimizer = f(E::View::pure()).optimizer();
        optimizer.optimize(&mut self.query);
        self
    }

    pub fn map<F, R: Clone, P>(self, f: F) -> QueryResult<E, R>
        where
            F: Fn(E::View, ViewBox<V>) -> ViewBox<R>,
    {
        let new_view = f(E::View::pure(), self.current_view);
        QueryResult {
            query: self.query,
            current_view: new_view,
            _marker: PhantomData,
        }
    }
}