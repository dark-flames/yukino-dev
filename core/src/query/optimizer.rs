use crate::query::queries::Query;

pub trait QueryOptimizer {
    fn optimize(&self, query: &mut Query);
}
