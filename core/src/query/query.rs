use crate::entity::EntityView;
use crate::expr::Expr;
use crate::query::calc::Computation;
use std::collections::HashMap;
use std::marker::PhantomData;

pub struct QueryResultRaw(HashMap<String, String>);

pub struct Query {}

impl Query {
    pub fn execute(&self) -> QueryResultRaw {
        unimplemented!()
    }
}

pub struct QueryResult<'f, E: EntityView, V: 'f + Clone> {
    query: Query,
    calculate: Computation<'f, V>,
    _marker: PhantomData<E>,
}

impl<'f, E: EntityView, V: 'f + Clone> QueryResult<'f, E, V> {
    pub fn eval(self) -> V {
        let result = self.query.execute();
        self.calculate.eval(&result)
    }

    pub fn map<F, R: 'f + Clone>(self, f: F) -> QueryResult<'f, E, R>
    where
        F: 'f + Fn(E, V) -> Box<dyn Expr<R>>,
    {
        QueryResult {
            query: self.query,
            calculate: self.calculate.bind(move |v| f(E::pure(), v).computation()),
            _marker: PhantomData,
        }
    }
}
