use std::marker::PhantomData;

use query_builder::SelectFrom;

use crate::view::EntityWithView;

pub struct DeleteQuery<E: EntityWithView> {
    _source: SelectFrom,
    _entity: PhantomData<E>
}

pub trait Delete<E: EntityWithView> {
    fn delete(self) -> DeleteQuery<E>;
}

impl<E: EntityWithView> DeleteQuery<E> {
    pub fn create(source: SelectFrom) -> Self {
        DeleteQuery {
            _source: source,
            _entity: PhantomData
        }
    }
}

// todo impl ExecutableSelectQuery<(), Empty> for DeleteQuery