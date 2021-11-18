use crate::query::AliasGenerator;
use crate::view::{EntityView, EntityWithView, ExprViewBox};
use query_builder::{Alias, SelectFrom};

pub struct QueryResultFilter<E: EntityWithView> {
    query: SelectFrom<E>,
    root_alias: Alias,
    alias_generator: AliasGenerator,
}

pub trait Filter {
    type View;

    fn filter<F, R: Into<ExprViewBox<bool>>>(&mut self, f: F)
    where
        F: Fn(&Self::View) -> R;
}

impl<E: EntityWithView> Filter for QueryResultFilter<E> {
    type View = E::View;
    fn filter<F, R: Into<ExprViewBox<bool>>>(&mut self, f: F)
    where
        F: Fn(&Self::View) -> R,
    {
        let entity_view = E::View::pure(&self.root_alias);
        let mut view = f(&entity_view).into();
        let mut visitor = self.alias_generator.substitute_visitor();
        view.apply_mut(&mut visitor);
        view.collect_expr().into_iter().for_each(|e| {
            self.query.and_where(e);
        });

        self.query.add_joins(visitor.joins());
    }
}
