use query_builder::SelectQuery;

use crate::view::{ExprViewBoxWithTag, TagList, Value};

pub struct SingleRow;
pub struct MultiRows;

pub trait ExecuteResultType {}

pub trait ExecutableSelectQuery<T: Value, TTags: TagList> {
    type ResultType: ExecuteResultType;

    fn generate_query(self) -> (SelectQuery, ExprViewBoxWithTag<T, TTags>);
}

impl ExecuteResultType for SingleRow {}
impl ExecuteResultType for MultiRows {}
