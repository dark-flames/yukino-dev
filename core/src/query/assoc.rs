use crate::query::QueryView;
use crate::view::EntityWithView;

pub trait SubSelectedEntityView<E: EntityWithView>: QueryView<E> {}

pub trait JoinedEntityView<E: EntityWithView>: QueryView<E> {}
