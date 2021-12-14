use query_builder::{InsertQuery, Query};

use crate::query::{Executable, SingleRow};
use crate::view::{ExprViewBoxWithTag, TagsOfValueView, Value};

impl Executable<(), TagsOfValueView<()>> for InsertQuery {
    type ResultType = SingleRow;

    fn generate_query(self) -> (Query, ExprViewBoxWithTag<(), TagsOfValueView<()>>) {
        (Query::Insert(self), ().view())
    }
}