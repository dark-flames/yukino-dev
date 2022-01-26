use std::collections::BTreeMap;
use std::hash::Hash;

use interface::{Association, FieldMarker, PrimaryKeyTypeOf, WithPrimaryKey};

use crate::query::FilteredQueryBuilder;
use crate::view::{EntityWithView, FieldMarkerWithView, TypeOfMarker, Value, ViewWithPrimaryKey};

pub trait AssociationBuilder<
    Children: EntityWithView + Association<Parent, ForeignField, ForeignKeyType = TypeOfMarker<ForeignField>>,
    Parent: EntityWithView + WithPrimaryKey<PrimaryKeyType = TypeOfMarker<ForeignField>>,
    ForeignField: FieldMarkerWithView + FieldMarker<Entity = Children>,
> where
    Parent::View: ViewWithPrimaryKey<PrimaryKeyType = TypeOfMarker<ForeignField>>,
    TypeOfMarker<ForeignField>: Value + Ord + Hash,
{
    fn build_query(self) -> FilteredQueryBuilder<Children>;

    fn build_from_parent_view(parent_view: &Parent::View) -> FilteredQueryBuilder<Children>;

    fn build_from_parent_entities(
        primary_keys: Vec<TypeOfMarker<ForeignField>>,
    ) -> FilteredQueryBuilder<Children>;
}

pub trait BelongsToQuery<Parent: EntityWithView>: EntityWithView {
    fn belonging_to_query<ForeignField: FieldMarkerWithView + FieldMarker<Entity = Self>>(
        r: FilteredQueryBuilder<Parent>,
    ) -> FilteredQueryBuilder<Self>
    where
        FilteredQueryBuilder<Parent>: AssociationBuilder<Self, Parent, ForeignField>,
        Parent: WithPrimaryKey<PrimaryKeyType = TypeOfMarker<ForeignField>>,
        Parent::View: ViewWithPrimaryKey<PrimaryKeyType = TypeOfMarker<ForeignField>>,
        Self:
            Sized + Association<Parent, ForeignField, ForeignKeyType = TypeOfMarker<ForeignField>>,
        TypeOfMarker<ForeignField>: Value + Ord + Hash,
    {
        r.build_query()
    }
}

pub trait BelongsToView<Parent: EntityWithView>: EntityWithView {
    fn belonging_to_view<ForeignField: FieldMarkerWithView + FieldMarker<Entity = Self>>(
        r: &Parent::View,
    ) -> FilteredQueryBuilder<Self>
    where
        FilteredQueryBuilder<Parent>: AssociationBuilder<Self, Parent, ForeignField>,
        Parent: WithPrimaryKey<PrimaryKeyType = TypeOfMarker<ForeignField>>,
        Parent::View: ViewWithPrimaryKey<PrimaryKeyType = TypeOfMarker<ForeignField>>,
        Self:
            Sized + Association<Parent, ForeignField, ForeignKeyType = TypeOfMarker<ForeignField>>,
        TypeOfMarker<ForeignField>: Value + Ord + Hash,
    {
        FilteredQueryBuilder::<Parent>::build_from_parent_view(r)
    }
}

pub trait BelongsToEntities<Parent: EntityWithView>: EntityWithView {
    fn belonging_to<ForeignField: FieldMarkerWithView + FieldMarker<Entity = Self>>(
        r: &[Parent],
    ) -> FilteredQueryBuilder<Self>
    where
        FilteredQueryBuilder<Parent>: AssociationBuilder<Self, Parent, ForeignField>,
        Parent: WithPrimaryKey<PrimaryKeyType = TypeOfMarker<ForeignField>>,
        Parent::View: ViewWithPrimaryKey<PrimaryKeyType = TypeOfMarker<ForeignField>>,
        Self:
            Sized + Association<Parent, ForeignField, ForeignKeyType = TypeOfMarker<ForeignField>>,
        TypeOfMarker<ForeignField>: Value + Ord + Hash,
    {
        FilteredQueryBuilder::<Parent>::build_from_parent_entities(
            r.iter().map(|i| i.primary_key().clone()).collect(),
        )
    }
}

pub trait RightSideData {
    type Children: EntityWithView;

    fn children(&self) -> &Self::Children;
}

impl<Children: EntityWithView> RightSideData for Children {
    type Children = Children;

    fn children(&self) -> &Self::Children {
        self
    }
}

macro_rules! impl_right_side_data_for_tuple {
    [$($param: ident),*] => {
        impl<Children: EntityWithView, $($param),* > RightSideData for (Children, $($param),*) {
            type Children = Children;

            fn children(&self) -> &Self::Children {
                &self.0
            }
        }
    };
}

impl_right_side_data_for_tuple![T1];
impl_right_side_data_for_tuple![T1, T2];
impl_right_side_data_for_tuple![T1, T2, T3];
impl_right_side_data_for_tuple![T1, T2, T3, T4];
impl_right_side_data_for_tuple![T1, T2, T3, T4, T5];

pub trait JoinChildren<
    Children: EntityWithView,
    Parent: EntityWithView,
    Right: RightSideData<Children = Children>,
>
{
    fn join<ForeignField: FieldMarkerWithView + FieldMarker<Entity = Children>>(
        self,
        right: Vec<Right>,
    ) -> Vec<(Parent, Vec<Right>)>
    where
        Children: Association<Parent, ForeignField, ForeignKeyType = TypeOfMarker<ForeignField>>,
        Parent: WithPrimaryKey<PrimaryKeyType = TypeOfMarker<ForeignField>>,
        TypeOfMarker<ForeignField>: Value + Ord + Hash;
}

impl<
        Children: EntityWithView,
        Parent: EntityWithView,
        Right: RightSideData<Children = Children>,
    > JoinChildren<Children, Parent, Right> for Vec<Parent>
{
    fn join<ForeignField: FieldMarkerWithView + FieldMarker<Entity = Children>>(
        self,
        right: Vec<Right>,
    ) -> Vec<(Parent, Vec<Right>)>
    where
        Children: Association<Parent, ForeignField, ForeignKeyType = TypeOfMarker<ForeignField>>,
        Parent: WithPrimaryKey<PrimaryKeyType = TypeOfMarker<ForeignField>>,
        TypeOfMarker<ForeignField>: Value + Ord + Hash,
    {
        let parent: BTreeMap<PrimaryKeyTypeOf<Parent>, Parent> = self
            .into_iter()
            .map(|p| (p.primary_key().clone(), p))
            .collect();

        let mut grouped_children: BTreeMap<PrimaryKeyTypeOf<Parent>, Vec<Right>> = parent
            .values()
            .map(|p| (p.primary_key().clone(), vec![]))
            .collect();

        for r_i in right {
            grouped_children
                .get_mut(r_i.children().foreign_key())
                .unwrap()
                .push(r_i);
        }

        parent
            .into_iter()
            .map(|(_, p)| p)
            .zip(grouped_children.into_iter().map(|(_, c)| c))
            .collect()
    }
}

impl<Children: EntityWithView, Parent: EntityWithView> BelongsToQuery<Parent> for Children {}

impl<Children: EntityWithView, Parent: EntityWithView> BelongsToView<Parent> for Children {}

impl<Children: EntityWithView, Parent: EntityWithView> BelongsToEntities<Parent> for Children {}
