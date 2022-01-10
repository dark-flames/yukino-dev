use std::fmt::{Display, Formatter, Result, Write};

use sqlx::Database;
use sqlx::database::HasArguments;
use sqlx::query::QueryAs;

use crate::{AppendToArgs, DatabaseValue, ToSql};

pub type Token = String;
pub type PlaceHolder = String;

#[derive(Default)]
pub struct QueryBuildState {
    params: Vec<DatabaseValue>,
    tokens: Vec<Token>,
}

impl QueryBuildState {
    pub fn create_param(&mut self, v: &DatabaseValue) -> PlaceHolder {
        self.params.push(v.clone());

        "?".to_string()
    }

    pub fn append_param(&mut self, v: &DatabaseValue) -> Result {
        let place_holder = self.create_param(v);

        write!(self, "{}", place_holder)
    }

    pub fn join<T: ToSql>(
        &mut self,
        items: &[T],
        separator: impl Fn(&mut Self) -> Result,
    ) -> Result {
        self.join_by(items, |s, item| item.to_sql(s), separator)
    }

    pub fn join_by<T>(
        &mut self,
        items: &[T],
        callback: impl Fn(&mut Self, &T) -> Result,
        separator: impl Fn(&mut Self) -> Result,
    ) -> Result {
        let last_index = items.len() - 1;

        for (index, item) in items.iter().enumerate() {
            callback(self, item)?;

            if last_index != index {
                separator(self)?;
            }
        }

        Ok(())
    }

    pub fn bind_args<'q, DB: Database, O>(
        self,
        query: QueryAs<'q, DB, O, <DB as HasArguments<'q>>::Arguments>,
    ) -> QueryAs<'q, DB, O, <DB as HasArguments<'q>>::Arguments>
    where
        DatabaseValue: AppendToArgs<'q, DB>,
    {
        self.params
            .into_iter()
            .fold(query, |q, param| param.bind_on(q))
    }
}

impl Write for QueryBuildState {
    fn write_str(&mut self, s: &str) -> Result {
        self.tokens.push(s.to_string());
        Ok(())
    }
}

impl Display for QueryBuildState {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let result = self.tokens.join(" ");
        Display::fmt(&result, f)
    }
}
