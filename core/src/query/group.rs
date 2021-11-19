use std::marker::PhantomData;

use query_builder::GroupSelect;

use crate::query::{AliasGenerator, ConcatList, ListConcat, SuitForGroupByList};
use crate::view::EntityWithView;

#[allow(dead_code)]
pub struct GroupedQueryResult<E: EntityWithView, G: SuitForGroupByList<Entity=E>> {
    query: GroupSelect<E>,
    alias_generator: AliasGenerator,
    _marker: PhantomData<G>,
}

pub trait GroupBy<E: EntityWithView> {
    fn group_by<Fields: SuitForGroupByList<Entity=E>>(self) -> GroupedQueryResult<E, Fields>;
}

impl<E: EntityWithView, G: SuitForGroupByList<Entity=E>> GroupedQueryResult<E, G> {
    pub fn create(query: GroupSelect<E>, alias_generator: AliasGenerator) -> Self {
        GroupedQueryResult {
            query,
            alias_generator,
            _marker: Default::default(),
        }
    }

    pub fn group_by<Fields: SuitForGroupByList<Entity=E> + ListConcat<E, G>>(
        self,
    ) -> GroupedQueryResult<E, ConcatList<Fields, G>>
        where
            ConcatList<Fields, G>: SuitForGroupByList,
    {
        let GroupedQueryResult {
            query,
            alias_generator,
            ..
        } = self;

        GroupedQueryResult {
            query,
            alias_generator,
            _marker: Default::default(),
        }
    }
}
