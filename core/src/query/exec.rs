use std::vec::IntoIter;

use async_trait::async_trait;
use generic_array::ArrayLength;
use sqlx::{Database, Error, Executor, FromRow, MySql, query};
use sqlx::query::Query;

use query_builder::{
    AppendToArgs, BindArgs, DatabaseValue, QueryBuildState, ResultRow, ToSql, YukinoQuery,
};

use crate::view::{ExprViewBoxWithTag, TagList, Value, ValueCountOf};

#[derive(Debug, Clone)]
pub struct SingleRow;
#[derive(Debug, Clone)]
pub struct MultiRows;

pub trait ExecuteResultType: Clone {}

pub trait Executable<T: Value, TTags: TagList, DB: Database> {
    type ResultType: ExecuteResultType;
    type Query: YukinoQuery<DB>;

    fn generate_query(self) -> (Self::Query, ExprViewBoxWithTag<T, TTags>);
}

impl ExecuteResultType for SingleRow {}
impl ExecuteResultType for MultiRows {}

#[async_trait]
pub trait FetchOne<T: Value, TTags: TagList>:
    Executable<T, TTags, MySql, ResultType = SingleRow>
{
    async fn exec<'c, 'e, E: 'e + Executor<'c, Database = MySql>>(
        self,
        executor: E,
    ) -> Result<T, Error>
    where
        Self: Sized,
        DatabaseValue: for<'q> AppendToArgs<'q, MySql>,
        <ValueCountOf<T> as ArrayLength<DatabaseValue>>::ArrayType: Unpin,
        ResultRow<ValueCountOf<T>>: for<'r> FromRow<'r, <MySql as Database>::Row>,
    {
        let (yukino_query, view) = self.generate_query();
        let mut state = QueryBuildState::default();
        yukino_query.to_sql(&mut state).unwrap();
        let raw_query = state.to_string();
        let query: Query<MySql, _> = query(&raw_query);
        let query_with_args = yukino_query.bind_args(query);
        let result = query_with_args.fetch_one(executor).await?;
        view.eval(ResultRow::<ValueCountOf<T>>::from_row(&result)?.into())
            .map_err(|e| Error::Decode(Box::new(e)))
    }
}

pub struct QueryResultIterator<DB: Database, T: Value, TTags: TagList> {
    view: ExprViewBoxWithTag<T, TTags>,
    query_result: IntoIter<<DB as Database>::Row>,
}

impl<DB: Database, T: Value, TTags: TagList> Iterator for QueryResultIterator<DB, T, TTags>
where
    for<'r> ResultRow<ValueCountOf<T>>: FromRow<'r, <DB as Database>::Row>,
{
    type Item = Result<T, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        self.query_result.next().map(|row| {
            ResultRow::<ValueCountOf<T>>::from_row(&row).and_then(|result_row| {
                self.view
                    .eval(result_row.into())
                    .map_err(|e| Error::Decode(Box::new(e)))
            })
        })
    }
}

impl<DB: Database, T: Value, TTags: TagList> QueryResultIterator<DB, T, TTags>
where
    Self: Iterator<Item = Result<T, Error>>,
{
    pub fn try_collect(self) -> Result<Vec<T>, Error> {
        self.collect()
    }
}

#[async_trait]
pub trait FetchMulti<T: Value, TTags: TagList>:
    Executable<T, TTags, MySql, ResultType = MultiRows>
{
    async fn exec<'c: 'e, 'e, E: 'e + Executor<'c, Database = MySql>>(
        self,
        executor: E,
    ) -> Result<QueryResultIterator<MySql, T, TTags>, Error>
    where
        Self: Sized,
        DatabaseValue: for<'q> AppendToArgs<'q, MySql>,
        <ValueCountOf<T> as ArrayLength<DatabaseValue>>::ArrayType: Unpin,
        ResultRow<ValueCountOf<T>>: for<'r> FromRow<'r, <MySql as Database>::Row>,
    {
        let (yukino_query, view) = self.generate_query();

        let mut state = QueryBuildState::default();
        yukino_query.to_sql(&mut state).unwrap();
        let query_str = state.to_string();

        let query_result = yukino_query
            .bind_args(query(&query_str))
            .fetch_all(executor)
            .await?
            .into_iter();

        Ok(QueryResultIterator { view, query_result })
    }
}

impl<T: Value, TTags: TagList, E: Executable<T, TTags, MySql, ResultType = SingleRow>>
    FetchOne<T, TTags> for E
{
}
impl<T: Value, TTags: TagList, E: Executable<T, TTags, MySql, ResultType = MultiRows>>
    FetchMulti<T, TTags> for E
{
}
