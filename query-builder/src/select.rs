use std::fmt::Debug;

use crate::{Alias, Expr, Join};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Order {
    Asc,
    Desc,
}

pub struct Select;

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct SelectFrom {
    table: String,
    root_alias: Alias,
    join: Vec<Join>,
    where_clauses: Vec<Expr>,
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct GroupSelect {
    base: SelectFrom,
    group_by: Vec<Expr>,
    having: Vec<Expr>,
}

#[derive(Debug)]
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

pub trait SelectSource: Debug + 'static {
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

    fn clone_source(&self) -> Box<dyn SelectSource>;
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

impl SelectSource for SelectFrom {
    fn clone_source(&self) -> Box<dyn SelectSource> {
        Box::new(self.clone())
    }
}

impl SelectSource for GroupSelect {
    fn clone_source(&self) -> Box<dyn SelectSource> {
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

impl Clone for SelectQuery {
    fn clone(&self) -> Self {
        SelectQuery {
            base: self.base.clone_source(),
            select: self.select.clone(),
            order_by: self.order_by.clone(),
            limit: self.limit,
            offset: self.offset,
        }
    }
}
