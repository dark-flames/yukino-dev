use std::collections::BTreeMap;
use std::hash::Hash;

use interface::{Association, FieldMarker, PrimaryKeyTypeOf, WithPrimaryKey};

use crate::operator::In;
use crate::query::QueryResultFilter;
use crate::view::{
    AssociatedView, EntityWithView, ExprBoxOfAssociatedView, FieldMarkerWithView, TagOfMarker,
    Value, ViewWithPrimaryKey,
};

pub trait AssociationBuilder<
    Children: EntityWithView + Association<Parent, ForeignField, ForeignKeyType = ForeignType>,
    Parent: EntityWithView + WithPrimaryKey<PrimaryKeyType = ForeignType>,
    ForeignField: FieldMarkerWithView + FieldMarker<Entity = Children, FieldType = ForeignType>,
    ForeignType: Value + Ord + Hash,
> where
    Parent::View: ViewWithPrimaryKey<PrimaryKeyType = ForeignType>,
{
    fn build_query(self) -> QueryResultFilter<Children>;

    fn build_from_parent_view(parent_view: &Parent::View) -> QueryResultFilter<Children>;

    fn build_from_parent_entities(primary_keys: Vec<ForeignType>) -> QueryResultFilter<Children>;
}

pub trait BelongsToQueryResult<
    Parent: EntityWithView + WithPrimaryKey<PrimaryKeyType = ForeignType>,
    ForeignField: FieldMarkerWithView + FieldMarker<Entity = Self, FieldType = ForeignType>,
    ForeignType: Value + Ord + Hash,
>: EntityWithView + Association<Parent, ForeignField, ForeignKeyType = ForeignType> where
    QueryResultFilter<Parent>: AssociationBuilder<Self, Parent, ForeignField, ForeignType>,
    Parent::View: ViewWithPrimaryKey<PrimaryKeyType = ForeignType>,
{
    fn belonging_to_query(r: QueryResultFilter<Parent>) -> QueryResultFilter<Self>
    where
        Self: Sized,
    {
        r.build_query()
    }
}

pub trait BelongsToView<
    Children: EntityWithView + Association<Parent, ForeignField, ForeignKeyType = ForeignType>,
    Parent: EntityWithView + WithPrimaryKey<PrimaryKeyType = ForeignType>,
    ForeignField: FieldMarkerWithView + FieldMarker<Entity = Children, FieldType = ForeignType>,
    ForeignType: Value + Ord + Hash,
> where
    QueryResultFilter<Parent>: AssociationBuilder<Children, Parent, ForeignField, ForeignType>,
    Parent::View: ViewWithPrimaryKey<PrimaryKeyType = ForeignType>,
{
    fn belonging_to_view(r: &Parent::View) -> QueryResultFilter<Children>
    where
        Self: Sized,
    {
        QueryResultFilter::<Parent>::build_from_parent_view(r)
    }
}

pub trait BelongsToEntities<
    Children: EntityWithView + Association<Parent, ForeignField, ForeignKeyType = ForeignType>,
    Parent: EntityWithView + WithPrimaryKey<PrimaryKeyType = ForeignType>,
    ForeignField: FieldMarkerWithView + FieldMarker<Entity = Children, FieldType = ForeignType>,
    ForeignType: Value + Ord + Hash,
> where
    QueryResultFilter<Parent>: AssociationBuilder<Children, Parent, ForeignField, ForeignType>,
    Parent::View: ViewWithPrimaryKey<PrimaryKeyType = ForeignType>,
{
    fn belonging_to(r: &[Parent]) -> QueryResultFilter<Children>
    where
        Self: Sized,
    {
        QueryResultFilter::<Parent>::build_from_parent_entities(
            r.iter().map(|i| i.primary_key().clone()).collect(),
        )
    }
}

pub trait JoinChildren<
    Children: EntityWithView + Association<Parent, ForeignField, ForeignKeyType = ForeignType>,
    Parent: EntityWithView + WithPrimaryKey<PrimaryKeyType = ForeignType>,
    ForeignField: FieldMarkerWithView + FieldMarker<Entity = Children, FieldType = ForeignType>,
    ForeignType: Value + Ord + Hash,
>
{
    fn join(self, children: Vec<Children>) -> Vec<(Parent, Vec<Children>)>;
}

impl<
        Children: EntityWithView + Association<Parent, ForeignField, ForeignKeyType = ForeignType>,
        Parent: EntityWithView + WithPrimaryKey<PrimaryKeyType = ForeignType>,
        ForeignField: FieldMarkerWithView + FieldMarker<Entity = Children, FieldType = ForeignType>,
        ForeignType: Value + Ord + Hash,
    > JoinChildren<Children, Parent, ForeignField, ForeignType> for Vec<Parent>
{
    fn join(self, children: Vec<Children>) -> Vec<(Parent, Vec<Children>)> {
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

impl<
        Children: EntityWithView + Association<Parent, ForeignField, ForeignKeyType = ForeignType>,
        Parent: EntityWithView + WithPrimaryKey<PrimaryKeyType = ForeignType>,
        ForeignField: FieldMarkerWithView + FieldMarker<Entity = Children, FieldType = ForeignType>,
        ForeignType: Value + Ord + Hash,
    > BelongsToQueryResult<Parent, ForeignField, ForeignType> for Children
where
    Parent::View: ViewWithPrimaryKey<PrimaryKeyType = ForeignType>,
    Children::View: AssociatedView<
        Parent,
        ForeignField,
        ForeignKeyType = ForeignType,
        ForeignKeyTags = TagOfMarker<ForeignField>,
    >,
    ExprBoxOfAssociatedView<Children::View, Parent, ForeignField>:
        In<<Parent as WithPrimaryKey>::PrimaryKeyType>,
{
}

impl<
        Children: EntityWithView + Association<Parent, ForeignField, ForeignKeyType = ForeignType>,
        Parent: EntityWithView + WithPrimaryKey<PrimaryKeyType = ForeignType>,
        ForeignField: FieldMarkerWithView + FieldMarker<Entity = Children, FieldType = ForeignType>,
        ForeignType: Value + Ord + Hash,
    > BelongsToView<Children, Parent, ForeignField, ForeignType> for Children
where
    Parent::View: ViewWithPrimaryKey<PrimaryKeyType = ForeignType>,
    Children::View: AssociatedView<
        Parent,
        ForeignField,
        ForeignKeyType = ForeignType,
        ForeignKeyTags = TagOfMarker<ForeignField>,
    >,
    ExprBoxOfAssociatedView<Children::View, Parent, ForeignField>:
        In<<Parent as WithPrimaryKey>::PrimaryKeyType>,
{
}

impl<
        Children: EntityWithView + Association<Parent, ForeignField, ForeignKeyType = ForeignType>,
        Parent: EntityWithView + WithPrimaryKey<PrimaryKeyType = ForeignType>,
        ForeignField: FieldMarkerWithView + FieldMarker<Entity = Children, FieldType = ForeignType>,
        ForeignType: Value + Ord + Hash,
    > BelongsToEntities<Children, Parent, ForeignField, ForeignType> for Children
where
    Parent::View: ViewWithPrimaryKey<PrimaryKeyType = ForeignType>,
    Children::View: AssociatedView<
        Parent,
        ForeignField,
        ForeignKeyType = ForeignType,
        ForeignKeyTags = TagOfMarker<ForeignField>,
    >,
    ExprBoxOfAssociatedView<Children::View, Parent, ForeignField>:
        In<<Parent as WithPrimaryKey>::PrimaryKeyType>,
{
}
