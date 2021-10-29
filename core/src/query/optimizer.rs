use crate::interface::FieldMarker;
use crate::query::queries::Query;

pub type OptimizerBox = Box<dyn QueryOptimizer>;

pub struct SelectAppendOptimizer(Vec<String>);

pub struct OptimizerCombinator(OptimizerBox, OptimizerBox);

pub trait QueryOptimizer {
    fn optimize(&self, query: &mut Query);
}

impl QueryOptimizer for SelectAppendOptimizer {
    fn optimize(&self, _query: &mut Query) {
        todo!()
    }
}

impl QueryOptimizer for OptimizerCombinator {
    fn optimize(&self, query: &mut Query) {
        self.0.optimize(query);
        self.1.optimize(query)
    }
}

impl OptimizerCombinator {
    pub fn create(first: OptimizerBox, second: OptimizerBox) -> Box<Self> {
        Box::new(OptimizerCombinator(first, second))
    }
}

impl SelectAppendOptimizer {
    pub fn create() -> Box<Self> {
        Box::new(SelectAppendOptimizer(vec![]))
    }

    pub fn append<F: FieldMarker>(&mut self) -> &mut Self {
        self.0.extend(F::definition().columns.keys().cloned());
        self
    }

    pub fn append_by_columns(&mut self, columns: Vec<String>) -> &mut Self {
        self.0.extend(columns);
        self
    }
}
