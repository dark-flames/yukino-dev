use async_trait::async_trait;
use generic_array::{ArrayLength, GenericArray};
use sqlx::{Database, Error, Executor, FromRow, IntoArguments, query_as_with};
use sqlx::database::HasArguments;

use query_builder::{AppendToArgs, DatabaseValue, Query, QueryBuildState, ToSql};

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
    async fn exec<'c, 'e, DB: Database, E: 'e + Executor<'e, Database = DB>>(
        self,
        executor: E,
    ) -> Result<T, Error>
    where
        Self: Sized,
        DatabaseValue: for<'q> AppendToArgs<'q, DB>,
        for<'q> <DB as HasArguments<'q>>::Arguments: IntoArguments<'q, DB>,
        <ValueCountOf<T> as ArrayLength<DatabaseValue>>::ArrayType: Unpin,
        GenericArray<DatabaseValue, ValueCountOf<T>>: for<'r> FromRow<'r, DB::Row>,
    {
        let (query, view) = self.generate_query();
        let mut state = QueryBuildState::default();
        query.to_sql(&mut state).unwrap();
        let query = state.to_string();
        let args = state.args::<DB>();
        let result: GenericArray<DatabaseValue, ValueCountOf<T>> =
            query_as_with(&query, args).fetch_one(executor).await?;

        view.eval(&result).map_err(|e| Error::Decode(Box::new(e)))
    }
}

#[async_trait]
pub trait FetchMulti<T: Value, TTags: TagList>:
    Executable<T, TTags, ResultType = MultiRows>
{
    async fn exec<'c, 'e, DB: Database, E: 'e + Executor<'e, Database = DB>>(
        self,
        executor: E,
    ) -> Result<Vec<T>, Error>
    where
        Self: Sized,
        DatabaseValue: for<'q> AppendToArgs<'q, DB>,
        for<'q> <DB as HasArguments<'q>>::Arguments: IntoArguments<'q, DB>,
        <ValueCountOf<T> as ArrayLength<DatabaseValue>>::ArrayType: Unpin,
        GenericArray<DatabaseValue, ValueCountOf<T>>: for<'r> FromRow<'r, DB::Row>,
    {
        let (query, view) = self.generate_query();
        let mut state = QueryBuildState::default();
        query.to_sql(&mut state).unwrap();
        let query = state.to_string();
        let args = state.args::<DB>();
        let result: Vec<GenericArray<DatabaseValue, ValueCountOf<T>>> =
            query_as_with(&query, args).fetch_all(executor).await?;

        result
            .into_iter()
            .map(|r| view.eval(&r).map_err(|e| Error::Decode(Box::new(e))))
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
