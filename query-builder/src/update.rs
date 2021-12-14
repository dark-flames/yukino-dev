use std::fmt::Write;

use crate::{Alias, AliasedTable, Expr, OrderByItem, QueryBuildState, ToSql};

pub struct Update;

pub struct AssignmentItem {
    pub column: String,
    pub value: AssignmentValue
}

pub enum AssignmentValue {
    Expr(Expr),
    Default
}

pub struct UpdateQuery {
    from: AliasedTable,
    where_clauses: Vec<Expr>,
    limit: Option<usize>,
    order_by: Vec<OrderByItem>,
    assignments: Vec<AssignmentItem>
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
            assignments: Default::default()
        }
    }
}

impl UpdateQuery {
    pub fn root_alias(&self) -> &Alias {
        &self.from.alias
    }

    pub fn set(&mut self, column: String, value: AssignmentValue) -> &mut Self {
        self.assignments.push(AssignmentItem {
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

impl ToSql for AssignmentValue {
    fn to_sql(&self, state: &mut QueryBuildState) -> std::fmt::Result {
        match self {
            AssignmentValue::Expr(e) => e.to_sql(state),
            AssignmentValue::Default => write!(state, "DEFAULT")
        }
    }
}

impl ToSql for AssignmentItem {
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
        state.join(&self.assignments, |s| write!(s, ","))?;

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