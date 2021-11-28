use query_builder::{Order, OrderByItem};

use crate::query::{ExprMutVisitor, ExprNode, ExprVisitor};
use crate::view::{ExprViewBoxWithTag, InList, OrdViewTag, TagList, Value};

pub struct SortItem<T: Value, TTags: TagList> {
    expr: ExprViewBoxWithTag<T, TTags>,
    order: Order,
}

pub trait SortResult: ExprNode {
    fn order_by_items(&self) -> Vec<OrderByItem>;
}

pub trait Sort<View> {
    type Result;
    fn sort<R: SortResult, F: Fn(View) -> R>(self, f: F) -> Self::Result;
}

impl<T: Value, TTags: TagList> ExprNode for SortItem<T, TTags> {
    fn apply(&self, visitor: &mut dyn ExprVisitor) {
        self.expr.apply(visitor)
    }

    fn apply_mut(&mut self, visitor: &mut dyn ExprMutVisitor) {
        self.expr.apply_mut(visitor)
    }
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
