use query_builder::{Insert, InsertQuery, Query};

use crate::query::{Executable, MultiRows};
use crate::view::{EntityWithView, ExprViewBoxWithTag, Insertable, TagsOfValueView, Value};

impl Executable<(), TagsOfValueView<()>> for InsertQuery {
    type ResultType = MultiRows;

    fn generate_query(self) -> (Query, ExprViewBoxWithTag<(), TagsOfValueView<()>>) {
        (Query::Insert(self), ().view())
    }
}

pub trait BatchInsert {
    fn insert_all(self) -> InsertQuery;
}

impl<E: EntityWithView, InsertObject: Insertable<Entity=E>, List: IntoIterator<Item =InsertObject>> BatchInsert for List {
    fn insert_all(self) -> InsertQuery {
        let mut query = Insert::into(E::table_name().to_string(), InsertObject::columns());

        query.set(self.into_iter().map(|e| e.values()).collect());

        query
    }
}
