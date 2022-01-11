use std::fmt::{Display, Formatter, Write};
use std::marker::PhantomData;

use sqlx::Database;
use sqlx::database::HasArguments;
use sqlx::query::QueryAs;

use crate::{AppendToArgs, ArgSourceList, BindArgs, DatabaseValue, Query, QueryBuildState, ToSql};

pub struct Insert;

pub struct InsertQuery<DB: Database, O, S: for<'q> ArgSourceList<'q, DB, O>> {
    table: String,
    columns: Vec<String>,
    values: S,
    _db: PhantomData<(DB, O)>,
}

unsafe impl<DB: Database, O, S: for<'q> ArgSourceList<'q, DB, O>> Send for InsertQuery<DB, O, S> {}
unsafe impl<DB: Database, O, S: for<'q> ArgSourceList<'q, DB, O>> Sync for InsertQuery<DB, O, S> {}

impl Insert {
    pub fn into<DB: Database, O, S: for<'q> ArgSourceList<'q, DB, O>>(
        table: String,
        columns: Vec<String>,
        values: S,
    ) -> InsertQuery<DB, O, S> {
        InsertQuery {
            table,
            columns,
            values,
            _db: Default::default(),
        }
    }
}

impl<DB: Database, O, S: for<'q> ArgSourceList<'q, DB, O>> ToSql for InsertQuery<DB, O, S> {
    fn to_sql(&self, state: &mut QueryBuildState) -> std::fmt::Result {
        write!(state, "INSERT INTO {} (", self.table)?;
        state.join_by(&self.columns, |s, c| write!(s, "{}", c), |s| write!(s, ","))?;

        write!(state, ") VALUES {} ;", self.values.query_part())
    }
}

impl<'q, DB: Database, O, S: for<'p> ArgSourceList<'p, DB, O>> BindArgs<'q, DB, O>
    for InsertQuery<DB, O, S>
where
    DatabaseValue: for<'p> AppendToArgs<'p, DB>,
{
    fn bind_args(
        self,
        query: QueryAs<'q, DB, O, <DB as HasArguments<'q>>::Arguments>,
    ) -> QueryAs<'q, DB, O, <DB as HasArguments<'q>>::Arguments> {
        self.values.bind_args(query)
    }
}

impl<DB: Database, O, S: for<'q> ArgSourceList<'q, DB, O>> Display for InsertQuery<DB, O, S>
where
    DatabaseValue: for<'q> AppendToArgs<'q, DB>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut state = QueryBuildState::default();
        self.to_sql(&mut state)?;
        Display::fmt(state.to_string().as_str(), f)
    }
}

impl<DB: Database, O, S: for<'q> ArgSourceList<'q, DB, O>> Query<DB, O> for InsertQuery<DB, O, S> where
    DatabaseValue: for<'q> AppendToArgs<'q, DB>
{
}
