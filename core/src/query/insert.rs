use sqlx::Database;

use query_builder::{ArgSourceList, Insert, InsertQuery, Query, ResultRow};

use crate::query::{Executable, MultiRows};
use crate::view::{EntityWithView, ExprViewBoxWithTag, Insertable, TagsOfValueView, Value, ValueCountOf};

impl<
    DB: Database,
    S: for<'q> ArgSourceList<'q, DB, ResultRow<ValueCountOf<()>>>
> Executable<(), TagsOfValueView<()>, DB> for InsertQuery<DB, ResultRow<ValueCountOf<()>>, S>
    where Self: Query<DB, ResultRow<ValueCountOf<()>>>
{
    type ResultType = MultiRows;
    type Query = Self;

    fn generate_query(self) -> (Self::Query, ExprViewBoxWithTag<(), TagsOfValueView<()>>) {
        (self, ().view())
    }
}

pub trait BatchInsert<DB: Database, O, S: for<'q> ArgSourceList<'q, DB, O>> {
    fn insert_all(self) -> InsertQuery<DB, O, S>;
}

impl<
        DB: Database, O,
        E: EntityWithView,
        InsertObject: Insertable<DB, O, Entity = E>,
        List: IntoIterator<Item = InsertObject>,
    > BatchInsert<DB, O, Vec<InsertObject>> for List
{
    fn insert_all(self) -> InsertQuery<DB, O, Vec<InsertObject>> {
        Insert::into(
            E::table_name().to_string(),
            InsertObject::columns(),
            self.into_iter().collect()
        )
    }
}
