use query_builder::{Order, OrderByItem};

use crate::view::{ExprViewBoxWithTag, InList, OrdViewTag, TagList, Value};

pub struct SortItem<T: Value, TTags: TagList> {
    expr: ExprViewBoxWithTag<T, TTags>,
    order: query_builder::Order,
}

impl<T: Value, TTags: TagList> SortResult for SortItem<T, TTags>
    where
        OrdViewTag: InList<TTags>,
{
    fn order_by_items(&self) -> Vec<OrderByItem> {
        self.expr
            .collect_expr()
            .into_iter()
            .map(|expr| OrderByItem {
                expr,
                order: self.order,
            })
            .collect()
    }
}

pub trait SortOrder<T: Value, TTags: TagList> {
    fn asc(self) -> SortItem<T, TTags>;

    fn desc(self) -> SortItem<T, TTags>;
}

impl<T: Value, TTags: TagList> SortOrder<T, TTags> for ExprViewBoxWithTag<T, TTags>
    where OrdViewTag: InList<TTags>
{
    fn asc(self) -> SortItem<T, TTags> {
        SortItem {
            expr: self,
            order: Order::Asc
        }
    }

    fn desc(self) -> SortItem<T, TTags> {
        SortItem {
            expr: self,
            order: Order::Desc
        }
    }
}

pub trait SortResult {
    fn order_by_items(&self) -> Vec<OrderByItem>;
}

impl<T1: Value, T1Tags: TagList, T2: Value, T2Tags: TagList> SortResult
    for (SortItem<T1, T1Tags>, SortItem<T2, T2Tags>)
where
    OrdViewTag: InList<T1Tags> + InList<T2Tags>,
{
    fn order_by_items(&self) -> Vec<OrderByItem> {
        self.0
            .order_by_items()
            .into_iter()
            .chain(self.1.order_by_items().into_iter())
            .collect()
    }
}
