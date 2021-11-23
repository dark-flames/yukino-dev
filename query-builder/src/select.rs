use std::marker::PhantomData;

use interface::YukinoEntity;

use crate::{Alias, Expr, Join};

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Order {
    Asc,
    Desc,
}

pub struct Select;

#[allow(dead_code)]
pub struct SelectFrom<E: YukinoEntity> {
    root_alias: Alias,
    join: Vec<Join>,
    where_clauses: Vec<Expr>,
    _marker: PhantomData<E>,
}

#[allow(dead_code)]
pub struct GroupSelect<E: YukinoEntity> {
    base: SelectFrom<E>,
    group_by: Vec<Expr>,
    having: Vec<Expr>,
}

#[allow(dead_code)]
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

pub struct OrderByItem {
    pub expr: Expr,
    pub order: Order,
}

pub trait SelectSource: 'static {
    fn select(self, items: Vec<SelectItem>) -> SelectQuery
    where
        Self: Sized,
    {
        SelectQuery {
            base: Box::new(self),
            select: items,
            order_by: vec![],
            limit: None,
            offset: 0,
        }
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
    pub fn from<E: YukinoEntity>(alias: Alias) -> SelectFrom<E> {
        SelectFrom {
            root_alias: alias,
            join: vec![],
            where_clauses: vec![],
            _marker: Default::default(),
        }
    }
}

impl<E: YukinoEntity> SelectFrom<E> {
    pub fn and_where(&mut self, expr: Expr) -> &mut Self {
        self.where_clauses.push(expr);

        self
    }

    pub fn add_joins(&mut self, joins: Vec<Join>) -> &mut Self {
        self.join.extend(joins);

        self
    }

    pub fn group_by(self, columns: Vec<Expr>) -> GroupSelect<E> {
        GroupSelect {
            base: self,
            group_by: columns,
            having: vec![],
        }
    }
}

impl<E: YukinoEntity> GroupSelect<E> {
    pub fn having(&mut self, conditions: Vec<Expr>) -> &mut Self {
        self.having.extend(conditions);

        self
    }
}

impl<E: YukinoEntity> SelectSource for SelectFrom<E> {}

impl<E: YukinoEntity> SelectSource for GroupSelect<E> {}

impl SelectQuery {
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
