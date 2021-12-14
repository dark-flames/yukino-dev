use std::fmt::{Display, Formatter, Write};

pub use backend::*;
pub use delete::*;
pub use expr::*;
pub use function::*;
pub use ident::*;
pub use insert::*;
pub use join::*;
pub use select::*;
pub use update::*;
pub use value::*;

mod backend;
mod drivers;
mod expr;
mod function;
mod ident;
mod join;
mod select;
mod value;
mod update;
mod delete;
mod insert;


pub enum Query {
    Select(SelectQuery),
    Update(UpdateQuery),
    Delete(DeleteQuery),
    Insert(InsertQuery)
}

impl ToSql for Query {
    fn to_sql(&self, state: &mut QueryBuildState) -> std::fmt::Result {
        match self {
            Query::Select(s) => s.to_sql(state),
            Query::Update(u) => u.to_sql(state),
            Query::Delete(d) => d.to_sql(state),
            Query::Insert(i) => i.to_sql(state)
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