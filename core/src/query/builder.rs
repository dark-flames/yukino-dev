use std::marker::PhantomData;
use crate::interface::{Entity, EntityView};
use crate::query::{Alias, Expr};
use crate::view::{ExprViewBox, Value};

pub struct QueryResultFilter<
    E: Entity
> {
    filter: Vec<Expr>,
    root_alias: Alias,
    _entity: PhantomData<E>
}
impl<
    E: Entity
> QueryResultFilter<E> {
    pub fn filter<F>(&mut self, f: F)
        where F: Fn(E::View) -> ExprViewBox<bool, <bool as Value>::L> {
        let view = f(E::View::pure(&self.root_alias));
        // join
        self.filter.extend(view.collect_expr());
    }
}