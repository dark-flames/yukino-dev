use std::fmt::Result;
use std::iter::repeat;

use sqlx::Database;
use sqlx::database::HasArguments;
use sqlx::query::QueryAs;

use crate::QueryBuildState;

pub trait ToSql {
    fn to_sql(&self, state: &mut QueryBuildState) -> Result;
}

pub type QueryOf<'q, DB, O> = QueryAs<'q, DB, O, <DB as HasArguments<'q>>::Arguments>;

pub trait ArgSource<'q, DB: Database, O> {
    fn insert_value_count() -> usize;

    fn bind_args(
        self,
        query: QueryOf<'q, DB, O>
    ) -> QueryOf<'q, DB, O> where Self: Sized;
}

pub trait ArgSourceList<'q, DB: Database, O>:  {
    fn query_part(&self) -> String;

    fn bind_args(
        self,
        query: QueryOf<'q, DB, O>
    ) -> QueryOf<'q, DB, O> where Self: Sized;
}

pub trait BindArgs<'q, DB: Database, O> {
    fn bind_args(
        self,
        query: QueryAs<'q, DB, O, <DB as HasArguments<'q>>::Arguments>,
    ) -> QueryAs<'q, DB, O, <DB as HasArguments<'q>>::Arguments>;
}

impl<'q, DB: Database, O, I: BindArgs<'q, DB, O>, L: IntoIterator<Item = I>> BindArgs<'q, DB, O> for L {
    fn bind_args(
        self,
        query: QueryAs<'q, DB, O, <DB as HasArguments<'q>>::Arguments>,
    ) -> QueryAs<'q, DB, O, <DB as HasArguments<'q>>::Arguments> {
        self.into_iter().fold(query, |q, i| i.bind_args(q))
    }
}

impl<'q, DB: Database, O, S: ArgSource<'q, DB, O>> ArgSourceList<'q, DB, O> for Vec<S> {
    fn query_part(&self) -> String {
        let row = format!(
            "({})",
            repeat("?")
                .take(S::insert_value_count())
                .collect::<Vec<_>>().join(",")
        );

        repeat(row).take(self.len()).collect::<Vec<_>>().join(",")
    }

    fn bind_args(self, query: QueryOf<'q, DB, O>) -> QueryOf<'q, DB, O> where Self: Sized {
        self.into_iter().fold(
            query,
            |q, l| l.bind_args(q)
        )
    }
}
