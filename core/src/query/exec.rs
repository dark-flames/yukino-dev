use query_builder::SelectQuery;

use crate::view::{ExprViewBoxWithTag, TagList, Value};

#[derive(Debug, Clone)]
pub struct SingleRow;
#[derive(Debug, Clone)]
pub struct MultiRows;

pub trait ExecuteResultType: Clone {}

pub trait ExecutableSelectQuery<T: Value, TTags: TagList> {
    type ResultType: ExecuteResultType;

    fn generate_query(self) -> (SelectQuery, ExprViewBoxWithTag<T, TTags>);
}

impl ExecuteResultType for SingleRow {}
impl ExecuteResultType for MultiRows {}
