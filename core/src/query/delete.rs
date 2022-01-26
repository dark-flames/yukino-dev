use std::marker::PhantomData;

use sqlx::Database;

use query_builder::{DeleteQuery, OrderByItem, SelectFrom, YukinoQuery};

use crate::operator::SortResult;
use crate::query::{Executable, MultiRows, Sort};
use crate::view::{EntityView, EntityWithView};

pub struct DeletionBuilder<E: EntityWithView> {
    query: DeleteQuery,
    _entity: PhantomData<E>,
}

pub trait Delete<E: EntityWithView> {
    fn delete(self) -> DeletionBuilder<E>;
}

impl<E: EntityWithView> DeletionBuilder<E> {
    pub fn create(source: SelectFrom) -> Self {
        DeletionBuilder {
            query: source.into(),
            _entity: PhantomData,
        }
    }

    pub fn create_with_order(source: SelectFrom, order_by_items: Vec<OrderByItem>) -> Self {
        let mut result = Self::create(source);

        result.query.append_order_by(order_by_items);

        result
    }

    #[must_use]
    pub fn limit(mut self, l: usize) -> Self {
        self.query.limit(l);

        self
    }
}

impl<E: EntityWithView> Sort<E::View> for DeletionBuilder<E> {
    type Result = DeletionBuilder<E>;

    fn sort<R: SortResult, F: Fn(E::View) -> R>(mut self, f: F) -> Self::Result {
        let result = f(E::View::pure(self.query.root_alias()));

        self.query.append_order_by(result.order_by_items());

        self
    }
}

impl<E: EntityWithView, DB: Database> Executable<(), DB> for DeletionBuilder<E>
where
    DeleteQuery: YukinoQuery<DB>,
{
    type ResultType = MultiRows;
    type Query = DeleteQuery;

    fn generate_query(self) -> Self::Query {
        self.query
    }
}
