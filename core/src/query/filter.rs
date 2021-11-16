use crate::query::AliasGenerator;
use crate::view::{EntityView, EntityWithView, ExprViewBox};
use query_builder::{Alias, ExprNode, GroupSelect, SelectFrom};

pub struct QueryResultFilter<E: EntityWithView> {
    query: SelectFrom<E>,
    root_alias: Alias,
    alias_generator: AliasGenerator,
}

#[allow(dead_code)]
pub struct GroupQueryResult<E: EntityWithView> {
    query: GroupSelect<E>,
    root_alias: Alias,
}

impl<E: EntityWithView> QueryResultFilter<E> {
    pub fn filter<F>(&mut self, f: F)
        where
            F: Fn(E::View) -> ExprViewBox<bool>,
    {
        let view = f(E::View::pure(&self.root_alias));
        let mut visitor = self.alias_generator.substitute_visitor();
        view.collect_expr()
            .into_iter()
            .map(|mut e| {
                e.apply_mut(&mut visitor);
                e
            })
            .for_each(|e| {
                self.query.and_where(e);
            });

        self.query.add_joins(visitor.joins());
    }
}
