use crate::interface::FieldMarker;
use crate::query::queries::Query;

// todo: alias
#[derive(Default)]
pub struct SelectAppendOptimizer(Vec<String>);

pub trait QueryOptimizer {
    fn optimize(&self, query: &mut Query);
}

impl QueryOptimizer for SelectAppendOptimizer {
    fn optimize(&self, _query: &mut Query) {
        todo!()
    }
}

impl SelectAppendOptimizer {
    pub fn append<F: FieldMarker>(&mut self) -> &mut Self {
        self.0.extend(F::definition().columns.keys().cloned());
        self
    }
}
