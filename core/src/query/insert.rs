use sqlx::Database;

use query_builder::{ArgSourceList, Insert, InsertQuery, YukinoQuery};

use crate::query::{Executable, MultiRows};
use crate::view::{EntityWithView, ExprViewBoxWithTag, Insertable, TagsOfValueView, Value};

impl<DB: Database, S: for<'q> ArgSourceList<'q, DB>> Executable<(), TagsOfValueView<()>, DB>
    for InsertQuery<DB, S>
where
    Self: YukinoQuery<DB>,
{
    type ResultType = MultiRows;
    type Query = Self;

    fn generate_query(self) -> (Self::Query, ExprViewBoxWithTag<(), TagsOfValueView<()>>) {
        (self, ().view())
    }
}

pub trait BatchInsert<DB: Database, S: for<'q> ArgSourceList<'q, DB>> {
    fn insert_all(self) -> InsertQuery<DB, S>;
}

impl<
        DB: Database,
        E: EntityWithView,
        InsertObject: Insertable<DB, Entity = E>,
        List: IntoIterator<Item = InsertObject>,
    > BatchInsert<DB, Vec<InsertObject>> for List
{
    fn insert_all(self) -> InsertQuery<DB, Vec<InsertObject>> {
        Insert::into(
            E::table_name().to_string(),
            InsertObject::columns(),
            self.into_iter().collect(),
        )
    }
}
