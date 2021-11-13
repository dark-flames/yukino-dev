use crate::interface::{Entity, EntityView};
use crate::query::{Alias, Expr};
use crate::view::ExprViewBox;
use std::marker::PhantomData;

pub struct QueryResultFilter<E: Entity> {
    filter: Vec<Expr>,
    root_alias: Alias,
    _entity: PhantomData<E>,
}
impl<E: Entity> QueryResultFilter<E> {
    pub fn filter<F>(&mut self, f: F)
    where
        F: Fn(E::View) -> ExprViewBox<bool>,
    {
        let view = f(E::View::pure(&self.root_alias));
        // join
        self.filter.extend(view.collect_expr());
    }
}
