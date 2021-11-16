use crate::query::AliasGenerator;
use crate::view::{EntityView, EntityWithView, ExprViewBox};
use query_builder::{Alias, GroupSelect, SelectFrom};

pub struct QueryResultFilter<E: EntityWithView> {
    query: SelectFrom<E>,
    root_alias: Alias,
    _alias_generator: AliasGenerator,
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
        view.collect_expr()
            .into_iter()
            .map(|_e| {
                todo!("handle join");
                #[allow(unreachable_code)]
                    _e
            })
            .for_each(|e| {
                self.query.and_where(e);
            });
    }
}
