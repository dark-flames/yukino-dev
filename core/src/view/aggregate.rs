use generic_array::GenericArray;

use query_builder::AggregateFunctionCall;

use crate::view::{Value, ValueCountOf};

pub struct AggregateItem<T: Value> {
    pub exprs: GenericArray<Box<dyn AggregateFunctionCall>, ValueCountOf<T>>
}