use query_builder::{InsertQuery, Query};

use crate::query::{Executable, MultiRows};
use crate::view::{ExprViewBoxWithTag, TagsOfValueView, Value};

impl Executable<(), TagsOfValueView<()>> for InsertQuery {
    type ResultType = MultiRows;

    fn generate_query(self) -> (Query, ExprViewBoxWithTag<(), TagsOfValueView<()>>) {
        (Query::Insert(self), ().view())
    }
}
