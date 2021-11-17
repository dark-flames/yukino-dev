use crate::query::{AliasGenerator, Filter};
use crate::view::{EntityWithView, ExprView, ExprViewBox, Value};
use query_builder::{ExprVisitor, GroupSelect, Ident, SelectFrom};
use std::marker::PhantomData;

pub trait GroupBy<E: EntityWithView> {
    fn group_by<V0: Value, R: ExprView<V0>, F: Fn(E::View) -> R>(
        self,
        f: F,
    ) -> GroupedQueryResult<E, V0, R>;
}

pub struct GroupedQueryResult<E: EntityWithView, V0: Value, V0View: ExprView<V0>> {
    _alias_generator: AliasGenerator,
    query: GroupSelect<E>,
    grouped_view: V0View,
    _marker: PhantomData<V0>,
}

#[derive(Default)]
pub struct ColumnCollectVisitor {
    idents: Vec<Ident>,
}

impl ColumnCollectVisitor {
    pub fn unwrap(self) -> Vec<Ident> {
        self.idents
    }
}

impl ExprVisitor for ColumnCollectVisitor {
    fn visit_ident(&mut self, node: &Ident) {
        self.idents.push(node.clone())
    }
}

impl<E: EntityWithView, V0: Value, V0View: ExprView<V0>> GroupedQueryResult<E, V0, V0View> {
    pub fn create(
        query: SelectFrom<E>,
        alias_generator: AliasGenerator,
        grouped_view: V0View,
    ) -> Self {
        let mut visitor = ColumnCollectVisitor::default();
        grouped_view.apply(&mut visitor);
        let grouped_query = query.group_by(visitor.unwrap());

        GroupedQueryResult {
            _alias_generator: alias_generator,
            query: grouped_query,
            grouped_view,
            _marker: PhantomData,
        }
    }
}

impl<E: EntityWithView, V0: Value, V0View: ExprView<V0>> Filter
for GroupedQueryResult<E, V0, V0View>
{
    type View = V0View;

    fn filter<F, R: Into<ExprViewBox<bool>>>(&mut self, f: F)
        where
            F: Fn(&Self::View) -> R,
            Self: Sized,
    {
        let view = f(&self.grouped_view).into();
        self.query.having(view.collect_expr().into_iter().collect());
    }
}
