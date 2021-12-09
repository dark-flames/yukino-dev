use std::fmt::{Debug, Display, Formatter, Write};

use crate::{Alias, AliasedTable, Expr, Join, QueryBuildState, ToSql};

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
    base: Box<dyn SelectSource>,
    select: Vec<SelectItem>,
    order_by: Vec<OrderByItem>,
    limit: Option<usize>,
    offset: usize,
}

#[derive(Clone, Debug)]
pub struct SelectItem {
    pub expr: Expr,
    pub alias: String,
}

#[derive(Clone, Debug)]
pub struct OrderByItem {
    pub expr: Expr,
    pub order: Order,
}

pub trait SelectSource: ToSql + Display + Debug + 'static {
    fn box_clone(&self) -> Box<dyn SelectSource>;
    fn select(self, items: Vec<SelectItem>) -> SelectQuery
    where
        Self: Sized,
    {
        SelectQuery::create(Box::new(self), items, vec![], None, 0)
    }

    fn order_by(self, items: Vec<OrderByItem>) -> SelectQuery
    where
        Self: Sized,
    {
        SelectQuery {
            base: Box::new(self),
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
            base: Box::new(self),
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
            base: Box::new(self),
            select: vec![],
            order_by: vec![],
            limit: None,
            offset: o,
        }
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
                alias: root_alias
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

impl SelectSource for SelectFrom {
    fn box_clone(&self) -> Box<dyn SelectSource> {
        Box::new(self.clone())
    }
}

impl SelectSource for GroupSelect {
    fn box_clone(&self) -> Box<dyn SelectSource> {
        Box::new(self.clone())
    }
}

impl SelectQuery {
    pub fn create(
        base: Box<dyn SelectSource>,
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
        write!(f, "{} AS {}", self.expr, self.alias)
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
        write!(
            f,
            "FROM {} {} {}",
            self.table, join_clauses, where_clauses
        )
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

impl Clone for Box<dyn SelectSource> {
    fn clone(&self) -> Self {
        self.box_clone()
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

impl ToSql for SelectItem {
    fn to_sql(&self, state: &mut QueryBuildState) -> std::fmt::Result {
        self.expr.to_sql(state)?;

        write!(state, "AS {}", self.alias)
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
