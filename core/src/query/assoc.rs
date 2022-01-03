use std::collections::BTreeMap;
use std::hash::Hash;

use interface::{Association, FieldMarker, PrimaryKeyTypeOf, WithPrimaryKey};

use crate::query::QueryResultFilter;
use crate::view::{EntityWithView, FieldMarkerWithView, TypeOfMarker, Value, ViewWithPrimaryKey};

pub trait AssociationBuilder<
    Children: EntityWithView + Association<Parent, ForeignField, ForeignKeyType = TypeOfMarker<ForeignField>>,
    Parent: EntityWithView + WithPrimaryKey<PrimaryKeyType = TypeOfMarker<ForeignField>>,
    ForeignField: FieldMarkerWithView + FieldMarker<Entity = Children>,
> where
    Parent::View: ViewWithPrimaryKey<PrimaryKeyType = TypeOfMarker<ForeignField>>,
    TypeOfMarker<ForeignField>: Value + Ord + Hash
{
    fn build_query(self) -> QueryResultFilter<Children>;

    fn build_from_parent_view(parent_view: &Parent::View) -> QueryResultFilter<Children>;

    fn build_from_parent_entities(primary_keys: Vec<TypeOfMarker<ForeignField>>) -> QueryResultFilter<Children>;
}

pub trait BelongsToQueryResult<Parent: EntityWithView>: EntityWithView {
    fn belonging_to_query<
        ForeignField: FieldMarkerWithView + FieldMarker<Entity = Self>,
    >(
        r: QueryResultFilter<Parent>,
    ) -> QueryResultFilter<Self>
    where
        QueryResultFilter<Parent>: AssociationBuilder<Self, Parent, ForeignField>,
        Parent: WithPrimaryKey<PrimaryKeyType = TypeOfMarker<ForeignField>>,
        Parent::View: ViewWithPrimaryKey<PrimaryKeyType = TypeOfMarker<ForeignField>>,
        Self: Sized + Association<Parent, ForeignField, ForeignKeyType = TypeOfMarker<ForeignField>>,
        TypeOfMarker<ForeignField>: Value + Ord + Hash
    {
        r.build_query()
    }
}

pub trait BelongsToView<Parent: EntityWithView>: EntityWithView {
    fn belonging_to_view<
        ForeignField: FieldMarkerWithView + FieldMarker<Entity = Self>
    >(
        r: &Parent::View,
    ) -> QueryResultFilter<Self>
    where
        QueryResultFilter<Parent>: AssociationBuilder<Self, Parent, ForeignField>,
        Parent: WithPrimaryKey<PrimaryKeyType = TypeOfMarker<ForeignField>>,
        Parent::View: ViewWithPrimaryKey<PrimaryKeyType = TypeOfMarker<ForeignField>>,
        Self: Sized + Association<Parent, ForeignField, ForeignKeyType = TypeOfMarker<ForeignField>>,
        TypeOfMarker<ForeignField>: Value + Ord + Hash
    {
        QueryResultFilter::<Parent>::build_from_parent_view(r)
    }
}

pub trait BelongsToEntities<Parent: EntityWithView>: EntityWithView {
    fn belonging_to<
        ForeignField: FieldMarkerWithView + FieldMarker<Entity = Self>
    >(
        r: &[Parent],
    ) -> QueryResultFilter<Self>
    where
        QueryResultFilter<Parent>: AssociationBuilder<Self, Parent, ForeignField>,
        Parent: WithPrimaryKey<PrimaryKeyType = TypeOfMarker<ForeignField>>,
        Parent::View: ViewWithPrimaryKey<PrimaryKeyType = TypeOfMarker<ForeignField>>,
        Self: Sized + Association<Parent, ForeignField, ForeignKeyType = TypeOfMarker<ForeignField>>,
        TypeOfMarker<ForeignField>: Value + Ord + Hash
    {
        QueryResultFilter::<Parent>::build_from_parent_entities(
            r.iter().map(|i| i.primary_key().clone()).collect(),
        )
    }
}

pub trait JoinChildren<Children: EntityWithView, Parent: EntityWithView> {
    fn join<
        ForeignField: FieldMarkerWithView + FieldMarker<Entity = Children>
    >(
        self,
        children: Vec<Children>,
    ) -> Vec<(Parent, Vec<Children>)>
    where
        Children: Association<Parent, ForeignField, ForeignKeyType = TypeOfMarker<ForeignField>>,
        Parent: WithPrimaryKey<PrimaryKeyType = TypeOfMarker<ForeignField>>,
        TypeOfMarker<ForeignField>: Value + Ord + Hash;
}

impl<Children: EntityWithView, Parent: EntityWithView> JoinChildren<Children, Parent>
    for Vec<Parent>
{
    fn join<
        ForeignField: FieldMarkerWithView + FieldMarker<Entity = Children>
    >(
        self,
        children: Vec<Children>,
    ) -> Vec<(Parent, Vec<Children>)>
    where
        Children: Association<Parent, ForeignField, ForeignKeyType = TypeOfMarker<ForeignField>>,
        Parent: WithPrimaryKey<PrimaryKeyType = TypeOfMarker<ForeignField>>,
        TypeOfMarker<ForeignField>: Value + Ord + Hash
    {
        let parent: BTreeMap<PrimaryKeyTypeOf<Parent>, Parent> = self
            .into_iter()
            .map(|p| (p.primary_key().clone(), p))
            .collect();

        let mut grouped_children: BTreeMap<PrimaryKeyTypeOf<Parent>, Vec<Children>> = parent
            .values()
            .map(|p| (p.primary_key().clone(), vec![]))
            .collect();

        for child in children {
            grouped_children
                .get_mut(child.foreign_key())
                .unwrap()
                .push(child);
        }

        parent
            .into_iter()
            .map(|(_, p)| p)
            .zip(grouped_children.into_iter().map(|(_, c)| c))
            .collect()
    }
}

impl<Children: EntityWithView, Parent: EntityWithView> BelongsToQueryResult<Parent> for Children {}

impl<Children: EntityWithView, Parent: EntityWithView> BelongsToView<Parent> for Children {}

impl<Children: EntityWithView, Parent: EntityWithView> BelongsToEntities<Parent> for Children {}
