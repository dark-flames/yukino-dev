use std::fmt::{Display, Formatter, Write};
use std::marker::PhantomData;

use sqlx::Database;

use crate::{
    AppendToArgs, ArgSourceList, BindArgs, DatabaseValue, QueryBuildState, QueryOf, ToSql,
    YukinoQuery,
};

pub struct Insert;

pub struct InsertQuery<DB: Database, S: for<'q> ArgSourceList<'q, DB>> {
    table: String,
    columns: Vec<String>,
    values: S,
    _db: PhantomData<DB>,
}

unsafe impl<DB: Database, S: for<'q> ArgSourceList<'q, DB>> Send for InsertQuery<DB, S> {}
unsafe impl<DB: Database, S: for<'q> ArgSourceList<'q, DB>> Sync for InsertQuery<DB, S> {}

impl Insert {
    pub fn into<DB: Database, S: for<'q> ArgSourceList<'q, DB>>(
        table: String,
        columns: Vec<String>,
        values: S,
    ) -> InsertQuery<DB, S> {
        InsertQuery {
            table,
            columns,
            values,
            _db: Default::default(),
        }
    }
}

impl<DB: Database, S: for<'q> ArgSourceList<'q, DB>> ToSql for InsertQuery<DB, S> {
    fn to_sql(&self, state: &mut QueryBuildState) -> std::fmt::Result {
        write!(state, "INSERT INTO {} (", self.table)?;
        state.join_by(&self.columns, |s, c| write!(s, "{}", c), |s| write!(s, ","))?;

        write!(state, ") VALUES {} ;", self.values.query_part())
    }
}

impl<'q, DB: Database, S: for<'p> ArgSourceList<'p, DB>> BindArgs<'q, DB> for InsertQuery<DB, S>
where
    DatabaseValue: for<'p> AppendToArgs<'p, DB>,
{
    fn bind_args(self, query: QueryOf<'q, DB>) -> QueryOf<'q, DB> {
        self.values.bind_args(query)
    }
}

impl<DB: Database, S: for<'q> ArgSourceList<'q, DB>> Display for InsertQuery<DB, S>
where
    DatabaseValue: for<'q> AppendToArgs<'q, DB>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut state = QueryBuildState::default();
        self.to_sql(&mut state)?;
        Display::fmt(state.to_string().as_str(), f)
    }
}

impl<DB: Database, S: for<'q> ArgSourceList<'q, DB>> YukinoQuery<DB> for InsertQuery<DB, S> where
    DatabaseValue: for<'q> AppendToArgs<'q, DB>
{
}
