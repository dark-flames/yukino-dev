use std::fmt::Write;

use crate::{Alias, AliasedTable, Expr, OrderByItem, QueryBuildState, ToSql};

pub struct Update;

pub struct AssignItem {
    pub column: String,
    pub value: AssignValue
}

pub enum AssignValue {
    Expr(Expr),
    Default
}

pub struct UpdateQuery {
    from: AliasedTable,
    where_clauses: Vec<Expr>,
    limit: Option<usize>,
    order_by: Vec<OrderByItem>,
    set: Vec<AssignItem>
}

impl Update {
    pub fn from(table: String, alias: Alias) -> UpdateQuery {
        UpdateQuery {
            from: AliasedTable {
                table,
                alias
            },
            where_clauses: vec![],
            limit: None,
            order_by: vec![],
            set: Default::default()
        }
    }
}

impl UpdateQuery {
    pub fn root_alias(&self) -> &Alias {
        &self.from.alias
    }

    pub fn set(&mut self, column: String, value: AssignValue) -> &mut Self {
        self.set.push(AssignItem {
            column,
            value
        });

        self
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

impl ToSql for AssignValue {
    fn to_sql(&self, state: &mut QueryBuildState) -> std::fmt::Result {
        match self {
            AssignValue::Expr(e) => e.to_sql(state),
            AssignValue::Default => write!(state, "DEFAULT")
        }
    }
}

impl ToSql for AssignItem {
    fn to_sql(&self, state: &mut QueryBuildState) -> std::fmt::Result {
        write!(state, "{}=", self.column)?;
        self.value.to_sql(state)
    }
}

impl ToSql for UpdateQuery {
    fn to_sql(&self, state: &mut QueryBuildState) -> std::fmt::Result {
        write!(state, "UPDATE")?;

        self.from.to_sql(state)?;
        write!(state, "SET")?;
        state.join(&self.set, |s| write!(s, ","))?;

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