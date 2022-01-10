use std::fmt::Result;

use sqlx::Database;
use sqlx::database::HasArguments;
use sqlx::query::QueryAs;

use crate::{AppendToArgs, DatabaseValue, QueryBuildState};

pub trait ToSql {
    fn to_sql(&self, state: &mut QueryBuildState) -> Result;
}

pub trait BindArgs {
    fn bind_args<'q, DB: Database, O>(
        self,
        query: QueryAs<'q, DB, O, <DB as HasArguments<'q>>::Arguments>,
    ) -> QueryAs<'q, DB, O, <DB as HasArguments<'q>>::Arguments>
    where
        DatabaseValue: AppendToArgs<'q, DB>;
}

impl<I: BindArgs, L: IntoIterator<Item = I>> BindArgs for L {
    fn bind_args<'q, DB: Database, O>(
        self,
        query: QueryAs<'q, DB, O, <DB as HasArguments<'q>>::Arguments>,
    ) -> QueryAs<'q, DB, O, <DB as HasArguments<'q>>::Arguments>
    where
        DatabaseValue: AppendToArgs<'q, DB>,
    {
        self.into_iter().fold(query, |q, i| i.bind_args(q))
    }
}
