use std::fmt::Write;

use crate::{AssignmentValue, QueryBuildState, ToSql};

pub struct Insert;

pub struct InsertQuery {
    table: String,
    columns: Vec<String>,
    values: Vec<Vec<AssignmentValue>>,
}

unsafe impl Send for InsertQuery {}
unsafe impl Sync for InsertQuery {}

impl Insert {
    pub fn into(table: String, columns: Vec<String>) -> InsertQuery {
        InsertQuery {
            table,
            columns,
            values: vec![],
        }
    }
}

impl InsertQuery {
    pub fn append(&mut self, values: Vec<AssignmentValue>) -> &mut Self {
        self.values.push(values);

        self
    }

    pub fn set(&mut self, values: Vec<Vec<AssignmentValue>>) -> &mut Self {
        self.values = values;

        self
    }
}

impl ToSql for InsertQuery {
    fn to_sql(&self, state: &mut QueryBuildState) -> std::fmt::Result {
        write!(state, "INSERT INTO {} (", self.table)?;
        state.join_by(&self.columns, |s, c| write!(s, "{}", c), |s| write!(s, ","))?;

        write!(state, ") VALUES")?;

        state.join_by(
            &self.values,
            |s, values| {
                write!(s, "(")?;
                s.join(values, |s| write!(s, ","))?;
                write!(s, ")")
            },
            |s| write!(s, ","),
        )?;

        write!(state, ";")
    }
}
