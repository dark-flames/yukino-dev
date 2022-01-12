use std::fmt::{Debug, Display, Formatter, Write};

use sqlx::Database;

use crate::{
    Alias, AliasedTable, AppendToArgs, BindArgs, DatabaseValue, Delete, Expr, Join,
    QueryBuildState, QueryOf, ToSql, Update, UpdateQuery, YukinoQuery,
};
use crate::delete::DeleteQuery;

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum Order {
    Asc,
    Desc,
}

#[derive(Clone, Debug)]
pub struct Select;

#[derive(Clone, Debug)]
pub struct SelectFrom {
    table: AliasedTable,
    join: Vec<Join>,
    where_clauses: Vec<Expr>,
}

#[derive(Clone, Debug)]
pub struct GroupSelect {
    base: SelectFrom,
    group_by: Vec<Expr>,
    having: Vec<Expr>,
}

#[derive(Clone, Debug)]
pub struct SelectQuery {
    base: SelectSource,
    select: Vec<SelectItem>,
    order_by: Vec<OrderByItem>,
    limit: Option<usize>,
    offset: usize,
}

#[derive(Clone, Debug)]
pub struct SelectItem {
    pub expr: Expr,
    pub alias: Option<String>,
}

#[derive(Clone, Debug)]
pub struct OrderByItem {
    pub expr: Expr,
    pub order: Order,
}

#[derive(Clone, Debug)]
pub enum SelectSource {
    From(SelectFrom),
    Group(GroupSelect),
}

pub trait IntoSelectSource {
    fn source(self) -> SelectSource
    where
        Self: Sized;
    fn select(self, items: Vec<SelectItem>) -> SelectQuery
    where
        Self: Sized,
    {
        SelectQuery::create(self.source(), items, vec![], None, 0)
    }

    fn order_by(self, items: Vec<OrderByItem>) -> SelectQuery
    where
        Self: Sized,
    {
        SelectQuery {
            base: self.source(),
            select: vec![],
            order_by: items,
            limit: None,
            offset: 0,
        }
    }

    fn limit(self, l: usize) -> SelectQuery
    where
        Self: Sized,
    {
        SelectQuery {
            base: self.source(),
            select: vec![],
            order_by: vec![],
            limit: Some(l),
            offset: 0,
        }
    }

    fn offset(self, o: usize) -> SelectQuery
    where
        Self: Sized,
    {
        SelectQuery {
            base: self.source(),
            select: vec![],
            order_by: vec![],
            limit: None,
            offset: o,
        }
    }
}

impl IntoSelectSource for SelectSource {
    fn source(self) -> SelectSource
    where
        Self: Sized,
    {
        self
    }
}

impl IntoSelectSource for SelectFrom {
    fn source(self) -> SelectSource
    where
        Self: Sized,
    {
        SelectSource::From(self)
    }
}

impl IntoSelectSource for GroupSelect {
    fn source(self) -> SelectSource
    where
        Self: Sized,
    {
        SelectSource::Group(self)
    }
}

impl Select {
    pub fn from(table: String, alias: Alias) -> SelectFrom {
        SelectFrom::create(table, alias)
    }
}

impl SelectFrom {
    pub fn create(table: String, root_alias: Alias) -> Self {
        SelectFrom {
            table: AliasedTable {
                table,
                alias: root_alias,
            },
            join: vec![],
            where_clauses: vec![],
        }
    }
    pub fn and_where(&mut self, expr: Expr) -> &mut Self {
        self.where_clauses.push(expr);

        self
    }

    pub fn add_joins(&mut self, joins: Vec<Join>) -> &mut Self {
        self.join.extend(joins);

        self
    }

    pub fn group_by(self, columns: Vec<Expr>) -> GroupSelect {
        GroupSelect {
            base: self,
            group_by: columns,
            having: vec![],
        }
    }
}

impl GroupSelect {
    pub fn having(&mut self, conditions: Vec<Expr>) -> &mut Self {
        self.having.extend(conditions);

        self
    }
}

unsafe impl Send for SelectFrom {}
unsafe impl Sync for SelectFrom {}

unsafe impl Send for GroupSelect {}
unsafe impl Sync for GroupSelect {}

impl SelectQuery {
    pub fn create(
        base: SelectSource,
        select: Vec<SelectItem>,
        order_by: Vec<OrderByItem>,
        limit: Option<usize>,
        offset: usize,
    ) -> Self {
        SelectQuery {
            base,
            select,
            order_by,
            limit,
            offset,
        }
    }
    pub fn append_select(&mut self, items: Vec<SelectItem>) -> &mut Self {
        self.select.extend(items);

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

    pub fn offset(&mut self, o: usize) -> &mut Self {
        self.offset = o;

        self
    }
}

impl Display for Order {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Order::Asc => write!(f, "ASC"),
            Order::Desc => write!(f, "DESC"),
        }
    }
}

impl Display for SelectItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.expr)?;

        if let Some(alias) = &self.alias {
            write!(f, "AS {}", alias)?;
        }

        Ok(())
    }
}

impl Display for OrderByItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.expr, self.order)
    }
}

impl Display for SelectFrom {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let join_clauses = self
            .join
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(" ");
        let where_clauses = if self.where_clauses.is_empty() {
            "".to_string()
        } else {
            format!(
                "WHERE {}",
                self.where_clauses
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(" AND ")
            )
        };
        write!(f, "FROM {} {} {}", self.table, join_clauses, where_clauses)
    }
}

impl Display for GroupSelect {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let group_by_clauses = if self.group_by.is_empty() {
            "".to_string()
        } else {
            format!(
                "GROUP BY {}",
                self.group_by
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        };

        let having_clauses = if self.group_by.is_empty() {
            "".to_string()
        } else {
            format!(
                "HAVING {}",
                self.having
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(" AND ")
            )
        };
        write!(f, "{} {} {}", self.base, group_by_clauses, having_clauses)
    }
}

impl Display for SelectQuery {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let select_items = self
            .select
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(", ");
        let order_by_clauses = if self.order_by.is_empty() {
            "".to_string()
        } else {
            format!(
                "ORDER BY {}",
                self.order_by
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        };
        let limit_clause = self
            .limit
            .map(|l| format!("LIMIT {}", l))
            .unwrap_or_default();
        write!(
            f,
            "SELECT {} {} {} {} OFFSET {}",
            select_items, self.base, order_by_clauses, limit_clause, self.offset
        )
    }
}

impl Display for SelectSource {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SelectSource::From(from) => write!(f, "{}", from),
            SelectSource::Group(group) => write!(f, "{}", group),
        }
    }
}

impl ToSql for Order {
    fn to_sql(&self, state: &mut QueryBuildState) -> std::fmt::Result {
        match self {
            Order::Asc => write!(state, "ASC"),
            Order::Desc => write!(state, "DESC"),
        }
    }
}

impl ToSql for OrderByItem {
    fn to_sql(&self, state: &mut QueryBuildState) -> std::fmt::Result {
        self.expr.to_sql(state)?;
        self.order.to_sql(state)
    }
}

impl<'q, DB: Database> BindArgs<'q, DB> for OrderByItem
where
    DatabaseValue: for<'p> AppendToArgs<'p, DB>,
{
    fn bind_args(self, query: QueryOf<'q, DB>) -> QueryOf<'q, DB> {
        self.expr.bind_args(query)
    }
}

impl ToSql for SelectFrom {
    fn to_sql(&self, state: &mut QueryBuildState) -> std::fmt::Result {
        write!(state, "FROM")?;

        self.table.to_sql(state)?;

        self.join.iter().try_for_each(|j| j.to_sql(state))?;

        if !self.where_clauses.is_empty() {
            write!(state, "WHERE")?;

            state.join(&self.where_clauses, |s| write!(s, "AND"))?;
        }

        Ok(())
    }
}

impl ToSql for SelectSource {
    fn to_sql(&self, state: &mut QueryBuildState) -> std::fmt::Result {
        match self {
            SelectSource::From(from) => from.to_sql(state),
            SelectSource::Group(group) => group.to_sql(state),
        }
    }
}

impl<'q, DB: Database> BindArgs<'q, DB> for SelectSource
where
    DatabaseValue: for<'p> AppendToArgs<'p, DB>,
{
    fn bind_args(self, query: QueryOf<'q, DB>) -> QueryOf<'q, DB> {
        match self {
            SelectSource::From(from) => from.bind_args(query),
            SelectSource::Group(group) => group.bind_args(query),
        }
    }
}

impl<'q, DB: Database> BindArgs<'q, DB> for SelectFrom
where
    DatabaseValue: for<'p> AppendToArgs<'p, DB>,
{
    fn bind_args(self, query: QueryOf<'q, DB>) -> QueryOf<'q, DB> {
        self.where_clauses.bind_args(self.join.bind_args(query))
    }
}

impl ToSql for GroupSelect {
    fn to_sql(&self, state: &mut QueryBuildState) -> std::fmt::Result {
        self.base.to_sql(state)?;
        if !self.group_by.is_empty() {
            write!(state, "GROUP BY")?;

            state.join(&self.group_by, |s| write!(s, ","))?;

            if !self.having.is_empty() {
                write!(state, "HAVING")?;
                state.join(&self.having, |s| write!(s, "AND"))?;
            }
        };

        Ok(())
    }
}

impl<'q, DB: Database> BindArgs<'q, DB> for GroupSelect
where
    DatabaseValue: for<'p> AppendToArgs<'p, DB>,
{
    fn bind_args(self, query: QueryOf<'q, DB>) -> QueryOf<'q, DB> {
        self.having
            .bind_args(self.group_by.bind_args(self.base.bind_args(query)))
    }
}

impl ToSql for SelectItem {
    fn to_sql(&self, state: &mut QueryBuildState) -> std::fmt::Result {
        self.expr.to_sql(state)?;

        if let Some(alias) = &self.alias {
            write!(state, "AS {}", alias)?;
        }

        Ok(())
    }
}

impl<'q, DB: Database> BindArgs<'q, DB> for SelectItem
where
    DatabaseValue: for<'p> AppendToArgs<'p, DB>,
{
    fn bind_args(self, query: QueryOf<'q, DB>) -> QueryOf<'q, DB> {
        self.expr.bind_args(query)
    }
}

impl ToSql for SelectQuery {
    fn to_sql(&self, state: &mut QueryBuildState) -> std::fmt::Result {
        write!(state, "SELECT")?;

        state.join(&self.select, |s| write!(s, ","))?;

        self.base.to_sql(state)?;

        if !self.order_by.is_empty() {
            write!(state, "ORDER BY")?;

            state.join(&self.order_by, |s| write!(s, ","))?;
        }

        if let Some(limit) = &self.limit {
            write!(state, "LIMIT {}", limit)?;
        }

        if self.offset != 0 {
            write!(state, "OFFSET {}", self.offset)?;
        }

        Ok(())
    }
}

impl<'q, DB: Database> BindArgs<'q, DB> for SelectQuery
where
    DatabaseValue: for<'p> AppendToArgs<'p, DB>,
{
    fn bind_args(self, query: QueryOf<'q, DB>) -> QueryOf<'q, DB> {
        let after_select = self.select.bind_args(query);

        let after_source = self.base.bind_args(after_select);
        self.order_by.bind_args(after_source)
    }
}

impl From<SelectFrom> for UpdateQuery {
    fn from(s: SelectFrom) -> Self {
        let mut result = Update::from(s.table.table, s.table.alias);

        for where_clause in s.where_clauses {
            result.and_where(where_clause);
        }

        result
    }
}

impl From<SelectFrom> for DeleteQuery {
    fn from(s: SelectFrom) -> Self {
        let mut result = Delete::from(s.table.table, s.table.alias);

        for where_clause in s.where_clauses {
            result.and_where(where_clause);
        }

        result
    }
}

impl<DB: Database> YukinoQuery<DB> for SelectQuery where DatabaseValue: for<'q> AppendToArgs<'q, DB> {}
