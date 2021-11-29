use query_builder::SelectQuery;

use crate::view::{ValueCount, ViewBox};

pub struct SingleRow;
pub struct MultiRows;

pub trait ExecuteResultType {}

pub trait ExecutableSelectQuery<T: 'static, L: ValueCount> {
    type ResultType: ExecuteResultType;

    fn generate_query(self) -> (SelectQuery, ViewBox<T, L>);
}

impl ExecuteResultType for SingleRow {}
impl ExecuteResultType for MultiRows {}
