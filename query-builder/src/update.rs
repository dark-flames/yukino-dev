use std::fmt::{Display, Formatter, Write};

use sqlx::Database;
use sqlx::database::HasArguments;
use sqlx::query::QueryAs;

use crate::{
    Alias, AliasedTable, AppendToArgs, BindArgs, DatabaseValue, Expr, OrderByItem, Query,
    QueryBuildState, ToSql,
};

pub struct Update;

pub struct AssignmentItem {
    pub column: String,
    pub value: AssignmentValue,
}

unsafe impl Send for AssignmentItem {}
unsafe impl Sync for AssignmentItem {}

pub enum AssignmentValue {
    Expr(Box<Expr>),
    Default,
}

unsafe impl Send for AssignmentValue {}
unsafe impl Sync for AssignmentValue {}

pub struct UpdateQuery {
    from: AliasedTable,
    where_clauses: Vec<Expr>,
    limit: Option<usize>,
    order_by: Vec<OrderByItem>,
    assignments: Vec<AssignmentItem>,
}

unsafe impl Send for UpdateQuery {}
unsafe impl Sync for UpdateQuery {}

impl Update {
    pub fn from(table: String, alias: Alias) -> UpdateQuery {
        UpdateQuery {
            from: AliasedTable { table, alias },
            where_clauses: vec![],
            limit: None,
            order_by: vec![],
            assignments: Default::default(),
        }
    }
}

impl UpdateQuery {
    pub fn root_alias(&self) -> &Alias {
        &self.from.alias
    }

    pub fn set(&mut self, column: String, value: AssignmentValue) -> &mut Self {
        self.assignments.push(AssignmentItem { column, value });

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
            AssignmentValue::Default => write!(state, "DEFAULT"),
        }
    }
}

impl<'q, DB: Database, O> BindArgs<'q, DB, O> for AssignmentValue
where
    DatabaseValue: for<'p> AppendToArgs<'p, DB>,
{
    fn bind_args(
        self,
        query: QueryAs<'q, DB, O, <DB as HasArguments<'q>>::Arguments>,
    ) -> QueryAs<'q, DB, O, <DB as HasArguments<'q>>::Arguments> {
        if let AssignmentValue::Expr(e) = self {
            e.bind_args(query)
        } else {
            query
        }
    }
}

impl ToSql for AssignmentItem {
    fn to_sql(&self, state: &mut QueryBuildState) -> std::fmt::Result {
        let column = format!("`{}`", self.column);
        write!(state, "{}=", column)?;
        self.value.to_sql(state)
    }
}

impl<'q, DB: Database, O> BindArgs<'q, DB, O> for AssignmentItem
where
    DatabaseValue: for<'p> AppendToArgs<'p, DB>,
{
    fn bind_args(
        self,
        query: QueryAs<'q, DB, O, <DB as HasArguments<'q>>::Arguments>,
    ) -> QueryAs<'q, DB, O, <DB as HasArguments<'q>>::Arguments> {
        self.value.bind_args(query)
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

impl<'q, DB: Database, O> BindArgs<'q, DB, O> for UpdateQuery
where
    DatabaseValue: for<'p> AppendToArgs<'p, DB>,
{
    fn bind_args(
        self,
        query: QueryAs<'q, DB, O, <DB as HasArguments<'q>>::Arguments>,
    ) -> QueryAs<'q, DB, O, <DB as HasArguments<'q>>::Arguments> {
        self.order_by.bind_args(self.where_clauses.bind_args(query))
    }
}

impl Display for UpdateQuery {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut state = QueryBuildState::default();
        self.to_sql(&mut state)?;
        Display::fmt(state.to_string().as_str(), f)
    }
}

impl<DB: Database, O> Query<DB, O> for UpdateQuery where DatabaseValue: for<'q> AppendToArgs<'q, DB> {}
