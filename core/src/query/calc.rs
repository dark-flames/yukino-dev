use crate::query::optimizer::QueryOptimizer;
use crate::query::query::QueryResultRaw;

pub struct Computation<'f, V: 'f + Clone> {
    run: Box<dyn 'f + Fn(&QueryResultRaw) -> V>,
}

impl<'f, V: 'f + Clone> Computation<'f, V> {
    pub fn pure(v: V) -> Self {
        Computation {
            run: Box::new(move |_| v.clone()),
        }
    }
    pub fn bind<F, R>(self, f: F) -> Computation<'f, R>
    where
        F: 'f + Fn(V) -> Computation<'f, R>,
        R: 'f + Clone,
    {
        Computation {
            run: Box::new(move |query_result| (f((*self.run)(query_result)).run)(query_result)),
        }
    }

    pub fn eval(&self, query_result: &QueryResultRaw) -> V {
        (*self.run)(query_result)
    }

    pub fn optimizer(&self) -> Box<dyn QueryOptimizer> {
        unimplemented!()
    }
}
