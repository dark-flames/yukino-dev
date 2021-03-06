use std::fmt::{Display, Formatter, Result as FmtResult, Write};

use sqlx::Database;

use crate::{AppendToArgs, BindArgs, DatabaseValue, Expr, QueryBuildState, QueryOf, ToSql};

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Ident {
    pub seg: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct Alias {
    pub name: String,
}

#[derive(Clone, Debug)]
pub struct AliasedTable {
    pub table: String,
    pub alias: Alias,
}

unsafe impl Send for Ident {}
unsafe impl Sync for Ident {}
unsafe impl Send for Alias {}
unsafe impl Sync for Alias {}
unsafe impl Send for AliasedTable {}
unsafe impl Sync for AliasedTable {}

impl Alias {
    pub fn create_ident(&self, column: &str) -> Ident {
        let mut ident = self.single_seg_ident();
        ident.append_str(column);
        ident
    }

    pub fn create_ident_expr(&self, column: &str) -> Expr {
        Expr::Ident(self.create_ident(column))
    }

    pub fn single_seg_ident(&self) -> Ident {
        Ident {
            seg: vec![self.name.clone()],
        }
    }
}

impl Ident {
    pub fn append_str(&mut self, column: &str) {
        self.seg.push(column.to_string())
    }

    pub fn extend(&mut self, ident: Ident) {
        self.seg.extend(ident.seg);
    }
}

impl Display for Ident {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.seg.join("."))
    }
}

impl Display for Alias {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        self.name.fmt(f)
    }
}

impl Display for AliasedTable {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{} {}", self.table, self.alias)
    }
}

impl ToSql for Ident {
    fn to_sql(&self, state: &mut QueryBuildState) -> FmtResult {
        let last_index = self.seg.len() - 1;
        for (index, item) in self.seg.iter().enumerate() {
            let ident_seg = format!("`{}`", item);
            state.write_str(&ident_seg)?;
            if index != last_index {
                write!(state, ".")?;
            }
        }

        Ok(())
    }
}

impl<'q, DB: Database> BindArgs<'q, DB> for Ident
where
    DatabaseValue: for<'p> AppendToArgs<'p, DB>,
{
    fn bind_args(self, query: QueryOf<'q, DB>) -> QueryOf<'q, DB> {
        query
    }
}

impl ToSql for AliasedTable {
    fn to_sql(&self, state: &mut QueryBuildState) -> FmtResult {
        write!(state, "{}", self.table)?;

        self.alias.to_sql(state)
    }
}

impl ToSql for Alias {
    fn to_sql(&self, state: &mut QueryBuildState) -> FmtResult {
        let alias = format!("`{}`", self.name);
        state.write_str(&alias)
    }
}
