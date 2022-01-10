use query_builder::{Insert, InsertQuery, Query};

use crate::query::{Executable, MultiRows};
use crate::view::{ExprViewBoxWithTag, Insertable, TagsOfValueView, Value};

impl Executable<(), TagsOfValueView<()>> for InsertQuery {
    type ResultType = MultiRows;

    fn generate_query(self) -> (Query, ExprViewBoxWithTag<(), TagsOfValueView<()>>) {
        (Query::Insert(self), ().view())
    }
}

pub trait BatchInsert {
    fn insert_all(self) -> InsertQuery;
}

impl<E: Insertable, I: IntoIterator<Item = E>> BatchInsert for I {
    fn insert_all(self) -> InsertQuery {
        let mut query = Insert::into(E::table_name().to_string(), E::columns());

        query.set(self.into_iter().map(|e| e.values()).collect());

        query
    }
}
