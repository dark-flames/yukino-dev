use std::marker::PhantomData;

use query_builder::{Alias, GroupSelect};

use crate::query::{AliasGenerator, ConcatList, ListConcat, SuitForGroupByList};
use crate::view::EntityWithView;

#[allow(dead_code)]
pub struct GroupedQueryResult<E: EntityWithView, G: SuitForGroupByList<Entity=E>> {
    query: GroupSelect<E>,
    alias_generator: AliasGenerator,
    root_alias: Alias,
    _marker: PhantomData<G>,
}

pub trait GroupBy<E: EntityWithView> {
    fn group_by<Fields: SuitForGroupByList<Entity=E>>(self) -> GroupedQueryResult<E, Fields>;
}

impl<E: EntityWithView, G: SuitForGroupByList<Entity=E>> GroupedQueryResult<E, G> {
    pub fn create(
        query: GroupSelect<E>,
        alias_generator: AliasGenerator,
        root_alias: Alias,
    ) -> Self {
        GroupedQueryResult {
            query,
            alias_generator,
            root_alias,
            _marker: Default::default(),
        }
    }

    pub fn group_by<Fields: SuitForGroupByList<Entity=E>>(
        self,
    ) -> GroupedQueryResult<E, ConcatList<G, Fields>>
        where
            G: ListConcat<E, Fields>,
            ConcatList<G, Fields>: SuitForGroupByList,
    {
        let GroupedQueryResult {
            mut query,
            alias_generator,
            root_alias,
            ..
        } = self;

        query.extend_group_by(
            Fields::idents(root_alias.single_seg_ident())
                .into_iter()
                .collect(),
        );

        GroupedQueryResult {
            query,
            alias_generator,
            root_alias,
            _marker: Default::default(),
        }
    }
}
