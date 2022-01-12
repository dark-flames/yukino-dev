use std::fmt::Result;
use std::iter::repeat;

use sqlx::Database;
use sqlx::database::HasArguments;
use sqlx::query::Query;

use crate::QueryBuildState;

pub trait ToSql {
    fn to_sql(&self, state: &mut QueryBuildState) -> Result;
}

pub type QueryOf<'q, DB> = Query<'q, DB, <DB as HasArguments<'q>>::Arguments>;

pub trait ArgSource<'q, DB: Database> {
    fn insert_value_count() -> usize;

    fn bind_args(self, query: QueryOf<'q, DB>) -> QueryOf<'q, DB>
    where
        Self: Sized;
}

pub trait ArgSourceList<'q, DB: Database> {
    fn query_part(&self) -> String;

    fn bind_args(self, query: QueryOf<'q, DB>) -> QueryOf<'q, DB>
    where
        Self: Sized;
}

pub trait BindArgs<'q, DB: Database> {
    fn bind_args(
        self,
        query: Query<'q, DB, <DB as HasArguments<'q>>::Arguments>,
    ) -> Query<'q, DB, <DB as HasArguments<'q>>::Arguments>;
}

impl<'q, DB: Database, I: BindArgs<'q, DB>, L: IntoIterator<Item = I>> BindArgs<'q, DB> for L {
    fn bind_args(self, query: QueryOf<'q, DB>) -> QueryOf<'q, DB> {
        self.into_iter().fold(query, |q, i| i.bind_args(q))
    }
}

impl<'q, DB: Database, S: ArgSource<'q, DB>> ArgSourceList<'q, DB> for Vec<S> {
    fn query_part(&self) -> String {
        let row = format!(
            "({})",
            repeat("?")
                .take(S::insert_value_count())
                .collect::<Vec<_>>()
                .join(",")
        );

        repeat(row).take(self.len()).collect::<Vec<_>>().join(",")
    }

    fn bind_args(self, query: QueryOf<'q, DB>) -> QueryOf<'q, DB>
    where
        Self: Sized,
    {
        self.into_iter().fold(query, |q, l| l.bind_args(q))
    }
}
