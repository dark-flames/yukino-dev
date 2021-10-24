use crate::db::ty::ValuePack;
use crate::err::RuntimeResult;
use crate::query::optimizer::QueryOptimizer;

pub struct Computation<'f, V: 'f + Clone> {
    run: Box<dyn 'f + Fn(&ValuePack) -> RuntimeResult<V>>,
}

impl<'f, V: 'f + Clone> Computation<'f, V> {
    pub fn create(run: Box<dyn 'f + Fn(&ValuePack) -> RuntimeResult<V>>) -> Self {
        Computation {
            run
        }
    }

    pub fn pure(v: V) -> Self {
        Computation {
            run: Box::new(move |_| Ok(v.clone())),
        }
    }
    pub fn bind<F, R>(self, f: F) -> Computation<'f, R>
    where
        F: 'f + Fn(V) -> Computation<'f, R>,
        R: 'f + Clone,
    {
        Computation {
            run: Box::new(move |query_result| (f((*self.run)(query_result)?).run)(query_result)),
        }
    }

    pub fn eval(&self, query_result: &ValuePack) -> RuntimeResult<V> {
        (*self.run)(query_result)
    }

    pub fn optimizer(&self) -> Box<dyn QueryOptimizer> {
        unimplemented!()
    }
}
