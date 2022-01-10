use std::fmt::{Display, Formatter, Write};

use sqlx::Database;
use sqlx::database::HasArguments;
use sqlx::query::QueryAs;

use crate::{
    AppendToArgs, BindArgs, DatabaseValue, DeleteQuery, InsertQuery, QueryBuildState, SelectQuery,
    ToSql, UpdateQuery,
};

pub enum Query {
    Select(SelectQuery),
    Update(UpdateQuery),
    Delete(DeleteQuery),
    Insert(InsertQuery),
}

unsafe impl Send for Query {}
unsafe impl Sync for Query {}

impl ToSql for Query {
    fn to_sql(&self, state: &mut QueryBuildState) -> std::fmt::Result {
        match self {
            Query::Select(s) => s.to_sql(state),
            Query::Update(u) => u.to_sql(state),
            Query::Delete(d) => d.to_sql(state),
            Query::Insert(i) => i.to_sql(state),
        }?;

        write!(state, ";")
    }
}

impl BindArgs for Query {
    fn bind_args<'q, DB: Database, O>(
        self,
        query: QueryAs<'q, DB, O, <DB as HasArguments<'q>>::Arguments>,
    ) -> QueryAs<'q, DB, O, <DB as HasArguments<'q>>::Arguments>
    where
        DatabaseValue: AppendToArgs<'q, DB>,
    {
        match self {
            Query::Select(s) => s.bind_args(query),
            Query::Update(u) => u.bind_args(query),
            Query::Delete(d) => d.bind_args(query),
            Query::Insert(i) => i.bind_args(query),
        }
    }
}

impl Display for Query {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Query::Select(s) = self {
            Display::fmt(s, f)
        } else {
            let mut state = QueryBuildState::default();
            self.to_sql(&mut state)?;
            Display::fmt(state.to_string().as_str(), f)
        }
    }
}
