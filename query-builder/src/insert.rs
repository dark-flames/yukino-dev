use std::fmt::Write;

use crate::{AssignmentItem, AssignmentValue, QueryBuildState, ToSql};

pub struct Insert;

pub struct InsertQuery {
    table: String,
    assignments: Vec<AssignmentItem>,
}

unsafe impl Send for InsertQuery {}
unsafe impl Sync for InsertQuery {}

impl Insert {
    pub fn into(table: String) -> InsertQuery {
        InsertQuery {
            table,
            assignments: vec![],
        }
    }
}

impl InsertQuery {
    pub fn set(&mut self, column: String, value: AssignmentValue) -> &mut Self {
        self.assignments.push(AssignmentItem { column, value });

        self
    }
}

impl ToSql for InsertQuery {
    fn to_sql(&self, state: &mut QueryBuildState) -> std::fmt::Result {
        write!(state, "INSERT INTO {} SET", self.table)?;

        state.join(&self.assignments, |s| write!(s, ","))
    }
}
