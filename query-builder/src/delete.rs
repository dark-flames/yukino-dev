use std::fmt::Write;

use crate::{Alias, AliasedTable, Expr, OrderByItem, QueryBuildState, ToSql};

pub struct Delete;

pub struct DeleteQuery {
    from: AliasedTable,
    where_clauses: Vec<Expr>,
    order_by: Vec<OrderByItem>,
    limit: Option<usize>
}

unsafe impl Send for DeleteQuery {}
unsafe impl Sync for DeleteQuery {}

impl Delete {
    pub fn from(table: String, alias: Alias) -> DeleteQuery {
        DeleteQuery {
            from: AliasedTable { table, alias },
            where_clauses: vec![],
            order_by: vec![],
            limit: None
        }
    }
}

impl DeleteQuery {
    pub fn root_alias(&self) -> &Alias {
        &self.from.alias
    }

    pub fn and_where(&mut self, expr: Expr) -> &mut Self {
        self.where_clauses.push(expr);

        self
    }

    pub fn append_order_by(&mut self, items: Vec<OrderByItem>) -> &mut Self {
        self.order_by.extend(items);

        self
    }

    pub fn limit(&mut self, l: usize) -> &mut Self {
        self.limit = Some(l);

        self
    }
}

impl ToSql for DeleteQuery {
    fn to_sql(&self, state: &mut QueryBuildState) -> std::fmt::Result {
        write!(state, "DELETE FROM {} AS", self.from.table)?;

        self.from.alias.to_sql(state)?;

        if !self.where_clauses.is_empty() {
            write!(state, "WHERE")?;
            state.join(&self.where_clauses, |s| write!(s, "AND"))?;
        }

        if !self.order_by.is_empty() {
            write!(state, "ORDER BY")?;
            state.join(&self.order_by, |s| write!(s, ","))?;
        }

        if let Some(l) = self.limit {
            write!(state, "LIMIT {}", l)?;
        }

        Ok(())
    }
}