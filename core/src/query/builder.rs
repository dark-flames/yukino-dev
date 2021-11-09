use crate::interface::Entity;
use crate::query::{Alias, Expr};
use crate::view::{Value, View, ViewBox};
use std::marker::PhantomData;

#[allow(dead_code)]
pub struct QueryBuilder<E: Entity, V: View<T>, T: Value> {
    filter: Box<Expr>,
    view: ViewBox<T>,
    from_alias: Alias,
    _view_marker: PhantomData<V>,
    _entity_marker: PhantomData<E>,
}

impl<E: Entity, V: View<T>, T: Value> QueryBuilder<E, V, T> {}
