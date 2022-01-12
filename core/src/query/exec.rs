use std::marker::PhantomData;
use std::vec::IntoIter;

use async_trait::async_trait;
use generic_array::{ArrayLength, typenum::U0};
use sqlx::{Database, Error, Executor, MySql, query};
use sqlx::query::Query;

use query_builder::{
    AppendToArgs, BindArgs, ColumnOf, DatabaseValue, QueryBuildState, ToSql, YukinoQuery,
};

use crate::view::{DBMapping, Value, ValueCountOf};

#[derive(Debug, Clone)]
pub struct SingleRow;
#[derive(Debug, Clone)]
pub struct MultiRows;

pub trait ExecuteResultType: Clone {}

pub trait Executable<T: Value, DB: Database> {
    type ResultType: ExecuteResultType;
    type Query: YukinoQuery<DB>;

    fn generate_query(self) -> Self::Query;
}

impl ExecuteResultType for SingleRow {}
impl ExecuteResultType for MultiRows {}

#[async_trait]
pub trait FetchOne<T: Value + for<'r> DBMapping<'r, MySql, U0>>:
    Executable<T, MySql, ResultType = SingleRow>
{
    async fn exec<'c, 'e, E: 'e + Executor<'c, Database = MySql>>(
        self,
        executor: E,
    ) -> Result<T, Error>
    where
        Self: Sized,
        DatabaseValue: for<'q> AppendToArgs<'q, MySql>,
        <ValueCountOf<T> as ArrayLength<DatabaseValue>>::ArrayType: Unpin,
        ValueCountOf<T>: for<'r> ArrayLength<ColumnOf<MySql>>,
    {
        let yukino_query = self.generate_query();
        let mut state = QueryBuildState::default();
        yukino_query.to_sql(&mut state).unwrap();
        let raw_query = state.to_string();
        let query: Query<MySql, _> = query(&raw_query);
        let query_with_args = yukino_query.bind_args(query);
        let row = query_with_args.fetch_one(executor).await?;

        T::from_result(&row)
    }
}

pub struct QueryResultIterator<DB: Database, T: Value> {
    query_result: IntoIter<<DB as Database>::Row>,
    _marker: PhantomData<T>,
}

impl<DB: Database, T: Value + for<'r> DBMapping<'r, DB, U0>> Iterator for QueryResultIterator<DB, T>
where
    ValueCountOf<T>: for<'r> ArrayLength<ColumnOf<DB>>,
{
    type Item = Result<T, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        self.query_result.next().map(|row| T::from_result(&row))
    }
}

impl<DB: Database, T: Value> QueryResultIterator<DB, T>
where
    Self: Iterator<Item = Result<T, Error>>,
{
    pub fn try_collect(self) -> Result<Vec<T>, Error> {
        self.collect()
    }
}

#[async_trait]
pub trait FetchMulti<T: Value + for<'r> DBMapping<'r, MySql, U0>>:
    Executable<T, MySql, ResultType = MultiRows>
{
    async fn exec<'c: 'e, 'e, E: 'e + Executor<'c, Database = MySql>>(
        self,
        executor: E,
    ) -> Result<QueryResultIterator<MySql, T>, Error>
    where
        Self: Sized,
        DatabaseValue: for<'q> AppendToArgs<'q, MySql>,
        <ValueCountOf<T> as ArrayLength<DatabaseValue>>::ArrayType: Unpin,
        ValueCountOf<T>: for<'r> ArrayLength<ColumnOf<MySql>>,
    {
        let yukino_query = self.generate_query();

        let mut state = QueryBuildState::default();
        yukino_query.to_sql(&mut state).unwrap();
        let query_str = state.to_string();

        let query_result = yukino_query
            .bind_args(query(&query_str))
            .fetch_all(executor)
            .await?
            .into_iter();

        Ok(QueryResultIterator {
            query_result,
            _marker: PhantomData,
        })
    }
}

impl<
        T: Value + for<'r> DBMapping<'r, MySql, U0>,
        E: Executable<T, MySql, ResultType = SingleRow>,
    > FetchOne<T> for E
{
}
impl<
        T: Value + for<'r> DBMapping<'r, MySql, U0>,
        E: Executable<T, MySql, ResultType = MultiRows>,
    > FetchMulti<T> for E
{
}
