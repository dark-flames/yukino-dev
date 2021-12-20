use std::fmt::{Display, Formatter, Write};

use crate::{DeleteQuery, InsertQuery, QueryBuildState, SelectQuery, ToSql, UpdateQuery};

pub enum Query {
    Select(SelectQuery),
    Update(UpdateQuery),
    Delete(DeleteQuery),
    Insert(InsertQuery),
}

unsafe impl Send for Query {}
unsafe impl Sync for Query {}

impl ToSql for Query {
    fn to_sql(&self, state: &mut QueryBuildState) -> std::fmt::Result {
        match self {
            Query::Select(s) => s.to_sql(state),
            Query::Update(u) => u.to_sql(state),
            Query::Delete(d) => d.to_sql(state),
            Query::Insert(i) => i.to_sql(state),
        }?;

        write!(state, ";")
    }
}

impl Display for Query {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Query::Select(s) = self {
            Display::fmt(s, f)
        } else {
            let mut state = QueryBuildState::default();
            self.to_sql(&mut state)?;
            Display::fmt(state.to_string().as_str(), f)
        }
    }
}
