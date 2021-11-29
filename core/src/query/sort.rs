use query_builder::{Order, OrderByItem};

use crate::query::{ExprMutVisitor, ExprNode, ExprVisitor};
use crate::view::{ExprViewBoxWithTag, InList, OrdViewTag, TagList, Value};

pub struct SortItem<T: Value, TTags: TagList> {
    expr: ExprViewBoxWithTag<T, TTags>,
    order: Order,
}

pub struct SortHelper;

pub trait SortResult: ExprNode {
    fn order_by_items(&self) -> Vec<OrderByItem>;
}

pub trait Sort<View> {
    type Result;
    fn sort<R: SortResult, F: Fn(View, SortHelper) -> R>(self, f: F) -> Self::Result;
}

pub trait Sort2<View1, View2> {
    type Result;
    fn sort<R: SortResult, F: Fn(View1, View2, SortHelper) -> R>(self, f: F) -> Self::Result;
}

impl<T: Value, TTags: TagList> ExprNode for SortItem<T, TTags> {
    fn apply(&self, visitor: &mut dyn ExprVisitor) {
        self.expr.apply(visitor);
    }

    fn apply_mut(&mut self, visitor: &mut dyn ExprMutVisitor) {
        self.expr.apply_mut(visitor);
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

impl<T1: Value, T1Tags: TagList, T2: Value, T2Tags: TagList> ExprNode
    for (SortItem<T1, T1Tags>, SortItem<T2, T2Tags>)
{
    fn apply(&self, visitor: &mut dyn ExprVisitor) {
        self.0.apply(visitor);
        self.1.apply(visitor);
    }

    fn apply_mut(&mut self, visitor: &mut dyn ExprMutVisitor) {
        self.0.apply_mut(visitor);
        self.1.apply_mut(visitor);
    }
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

impl SortHelper {
    pub(crate) fn create() -> Self {
        SortHelper
    }

    fn create_item<T: Value, TTags: TagList>(
        &self,
        view: impl Into<ExprViewBoxWithTag<T, TTags>>,
        order: Order,
    ) -> SortItem<T, TTags>
    where
        OrdViewTag: InList<TTags>,
    {
        let expr = view.into();
        SortItem { expr, order }
    }

    pub fn asc<T: Value, TTags: TagList>(
        &self,
        view: impl Into<ExprViewBoxWithTag<T, TTags>>,
    ) -> SortItem<T, TTags>
    where
        OrdViewTag: InList<TTags>,
    {
        self.create_item(view, Order::Asc)
    }

    pub fn desc<T: Value, TTags: TagList>(
        &self,
        view: impl Into<ExprViewBoxWithTag<T, TTags>>,
    ) -> SortItem<T, TTags>
    where
        OrdViewTag: InList<TTags>,
    {
        self.create_item(view, Order::Desc)
    }
}
