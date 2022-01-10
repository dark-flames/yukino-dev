use async_trait::async_trait;
use generic_array::ArrayLength;
use sqlx::{Database, Error, Executor, FromRow, MySql, query_as};
use sqlx::query::QueryAs;

use query_builder::{
    AppendToArgs, BindArgs, DatabaseValue, Query, QueryBuildState, ResultRow, ToSql,
};

use crate::view::{ExprViewBoxWithTag, TagList, Value, ValueCountOf};

#[derive(Debug, Clone)]
pub struct SingleRow;
#[derive(Debug, Clone)]
pub struct MultiRows;

pub trait ExecuteResultType: Clone {}

pub trait Executable<T: Value, TTags: TagList> {
    type ResultType: ExecuteResultType;

    fn generate_query(self) -> (Query, ExprViewBoxWithTag<T, TTags>);
}

impl ExecuteResultType for SingleRow {}
impl ExecuteResultType for MultiRows {}

#[async_trait]
pub trait FetchOne<T: Value, TTags: TagList>: Executable<T, TTags, ResultType = SingleRow> {
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
        let (query, view) = self.generate_query();
        let mut state = QueryBuildState::default();
        query.to_sql(&mut state).unwrap();
        let raw_query = state.to_string();
        let query_as: QueryAs<MySql, ResultRow<ValueCountOf<T>>, _> = query_as(&raw_query);
        let query_as = query.bind_args(query_as);
        let result = query_as.fetch_one(executor).await?;

        let arr = result.into();
        view.eval(&arr).map_err(|e| Error::Decode(Box::new(e)))
    }
}

#[async_trait]
pub trait FetchMulti<T: Value, TTags: TagList>:
    Executable<T, TTags, ResultType = MultiRows>
{
    async fn exec<'c, 'e, E: 'e + Executor<'c, Database = MySql>>(
        self,
        executor: E,
    ) -> Result<Vec<T>, Error>
    where
        Self: Sized,
        DatabaseValue: for<'q> AppendToArgs<'q, MySql>,
        <ValueCountOf<T> as ArrayLength<DatabaseValue>>::ArrayType: Unpin,
        ResultRow<ValueCountOf<T>>: for<'r> FromRow<'r, <MySql as Database>::Row>,
    {
        let (query, view) = self.generate_query();
        let mut state = QueryBuildState::default();
        query.to_sql(&mut state).unwrap();
        let raw_query = state.to_string();
        let query_as: QueryAs<MySql, ResultRow<ValueCountOf<T>>, _> = query_as(&raw_query);
        let query_as = query.bind_args(query_as);
        let result = query_as.fetch_all(executor).await?;
        result
            .into_iter()
            .map(|r| {
                let arr = r.into();
                view.eval(&arr).map_err(|e| Error::Decode(Box::new(e)))
            })
            .collect()
    }
}

impl<T: Value, TTags: TagList, E: Executable<T, TTags, ResultType = SingleRow>> FetchOne<T, TTags>
    for E
{
}
impl<T: Value, TTags: TagList, E: Executable<T, TTags, ResultType = MultiRows>> FetchMulti<T, TTags>
    for E
{
}
