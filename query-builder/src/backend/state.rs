use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result, Write};

use sqlx::Database;
use sqlx::database::HasArguments;

use crate::{AppendToArgs, DatabaseValue, ToSql};

pub type Token = String;
pub type PlaceHolder = String;

#[derive(Default)]
pub struct QueryBuildState {
    params: HashMap<usize, DatabaseValue>,
    counter: usize,
    tokens: Vec<Token>,
}

impl QueryBuildState {
    pub fn create_param(&mut self, v: &DatabaseValue) -> PlaceHolder {
        self.params.insert(self.counter, v.clone());
        let place_holder = format!("${}", self.counter);
        self.counter += 1;

        place_holder
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
        let last_index = items.len() - 1;

        for (index, item) in items.iter().enumerate() {
            item.to_sql(self)?;

            if last_index != index {
                separator(self)?;
            }
        }

        Ok(())
    }

    pub fn args<'q, DB: Database>(self) -> <DB as HasArguments<'q>>::Arguments
    where
        DatabaseValue: AppendToArgs<'q, DB>,
    {
        let mut result = DB::Arguments::default();

        for (_, value) in self.params.into_iter() {
            value.append_to(&mut result);
        }

        result
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
