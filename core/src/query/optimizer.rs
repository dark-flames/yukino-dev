use crate::query::query::Query;

pub trait QueryOptimizer {
    fn optimize(&self, query: &mut Query);
}
