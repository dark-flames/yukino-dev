use std::marker::PhantomData;

use query_builder::{DeleteQuery, OrderByItem, SelectFrom};

use crate::query::{Sort, SortHelper, SortResult};
use crate::view::{EntityView, EntityWithView};

pub struct DeleteQueryResult<E: EntityWithView> {
    query: DeleteQuery,
    _entity: PhantomData<E>
}

pub trait Delete<E: EntityWithView> {
    fn delete(self) -> DeleteQueryResult<E>;
}

impl<E: EntityWithView> DeleteQueryResult<E> {
    pub fn create(source: SelectFrom) -> Self {
        DeleteQueryResult {
            query: source.into(),
            _entity: PhantomData
        }
    }

    pub fn create_with_order(source: SelectFrom, order_by_items: Vec<OrderByItem>) -> Self {
        let mut result = Self::create(source);

        result.query.append_order_by(order_by_items);

        result
    }

    pub fn limit(mut self, l: usize) -> Self {
        self.query.limit(l);

        self
    }
}

impl<E: EntityWithView> Sort<E::View> for DeleteQueryResult<E> {
    type Result = DeleteQueryResult<E>;

    fn sort<R: SortResult, F: Fn(E::View, SortHelper) -> R>(mut self, f: F) -> Self::Result {
        let result = f(E::View::pure(self.query.root_alias()), SortHelper::create());

        self.query.append_order_by(result.order_by_items());

        self
    }
}

// todo impl ExecutableSelectQuery<(), Empty> for DeleteQuery