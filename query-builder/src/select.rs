use std::fmt::{Display, Formatter};

use crate::{Alias, Expr, Join};

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum Order {
    Asc,
    Desc,
}

pub struct Select;

pub struct SelectFrom {
    table: String,
    root_alias: Alias,
    join: Vec<Join>,
    where_clauses: Vec<Expr>,
}

pub struct GroupSelect {
    base: SelectFrom,
    group_by: Vec<Expr>,
    having: Vec<Expr>,
}

pub struct SelectQuery {
    base: Box<dyn SelectSource>,
    select: Vec<SelectItem>,
    order_by: Vec<OrderByItem>,
    limit: Option<usize>,
    offset: usize,
}

pub struct SelectItem {
    pub expr: Expr,
    pub alias: String,
}

#[derive(Clone, Debug)]
pub struct OrderByItem {
    pub expr: Expr,
    pub order: Order,
}

pub trait SelectSource: Display + 'static {
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
            table,
            root_alias,
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

impl SelectSource for SelectFrom {}

impl SelectSource for GroupSelect {}

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
            "FROM {} {} {} {}",
            self.table, self.root_alias, join_clauses, where_clauses
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
