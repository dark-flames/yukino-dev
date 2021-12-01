use std::marker::PhantomData;

use query_builder::{Order, OrderByItem};

use crate::view::{ExprViewBoxWithTag, HasTag, OrdViewTag, TagList, Value};

pub struct SortHelper(PhantomData<u16>);

pub trait SortResult {
    fn order_by_items(&self) -> Vec<OrderByItem>;
}

pub struct SortResultItem<T: Value, TTags: TagList> {
    view: ExprViewBoxWithTag<T, TTags>,
    order: Order,
}

impl SortHelper {
    pub(crate) fn create() -> Self {
        SortHelper(PhantomData)
    }

    pub fn asc<T: Value, TTags: TagList>(
        &self,
        view: ExprViewBoxWithTag<T, TTags>,
    ) -> SortResultItem<T, TTags> {
        SortResultItem {
            view,
            order: Order::Asc,
        }
    }

    pub fn desc<T: Value, TTags: TagList>(
        &self,
        view: ExprViewBoxWithTag<T, TTags>,
    ) -> SortResultItem<T, TTags> {
        SortResultItem {
            view,
            order: Order::Desc,
        }
    }
}

impl<T: Value, TTags: TagList> SortResult for SortResultItem<T, TTags>
where
    TTags: HasTag<OrdViewTag>,
{
    fn order_by_items(&self) -> Vec<OrderByItem> {
        self.view
            .collect_expr()
            .into_iter()
            .map(|expr| OrderByItem {
                expr,
                order: self.order,
            })
            .collect()
    }
}
