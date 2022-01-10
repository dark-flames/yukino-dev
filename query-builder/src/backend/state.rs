use std::fmt::{Display, Formatter, Result, Write};

use crate::ToSql;

pub type Token = String;
pub type PlaceHolder = String;

#[derive(Default)]
pub struct QueryBuildState {
    tokens: Vec<Token>,
}

impl QueryBuildState {
    pub fn append_param(&mut self) -> Result {
        write!(self, "?")
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
