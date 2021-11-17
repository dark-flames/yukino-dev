use crate::query::AliasGenerator;
use crate::view::{EntityView, EntityWithView, ExprViewBox};
use query_builder::{Alias, ExprNode, SelectFrom};

pub struct QueryResultFilter<E: EntityWithView> {
    query: SelectFrom<E>,
    root_alias: Alias,
    alias_generator: AliasGenerator,
}

impl<E: EntityWithView> QueryResultFilter<E> {
    pub fn filter<F, R: Into<ExprViewBox<bool>>>(&mut self, f: F)
        where
            F: Fn(E::View) -> R,
    {
        let view = f(E::View::pure(&self.root_alias)).into();
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
