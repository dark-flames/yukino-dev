use generic_array::GenericArray;
use sqlx::Database;

use interface::{Association, FieldMarker, WithPrimaryKey, YukinoEntity};
use query_builder::{Alias, ArgSource, ArgSourceList, InsertQuery};

use crate::query::{Delete, DeleteQueryResult, QueryResultFilter};
use crate::view::{ExprView, ExprViewBoxWithTag, TagList, Value, VerticalView};

pub trait EntityView: ExprView<Self::Entity> {
    type Entity: EntityWithView;

    fn pure(alias: &Alias) -> Self
    where
        Self: Sized;

    fn vertical(self) -> <Self::Entity as EntityWithView>::VerticalView
    where
        Self: Sized;
}

pub trait EntityVerticalView:
    VerticalView<Self::Entity, RowView = <Self::Entity as EntityWithView>::View>
{
    type Entity: EntityWithView;
}

pub trait EntityWithView: YukinoEntity + Value {
    type View: EntityView<Entity = Self>;
    type VerticalView: EntityVerticalView<Entity = Self>;
    type New;

    fn all() -> QueryResultFilter<Self>;
}

pub type ExprBoxOfAssociatedView<V, P, F> = ExprViewBoxWithTag<
    <V as AssociatedView<P, F>>::ForeignKeyType,
    <V as AssociatedView<P, F>>::ForeignKeyTags,
>;

pub type ExprBoxOfViewWithPrimaryKey<V> = ExprViewBoxWithTag<
    <V as ViewWithPrimaryKey>::PrimaryKeyType,
    <V as ViewWithPrimaryKey>::PrimaryKeyTags,
>;

pub trait ViewWithPrimaryKey: EntityView
where
    <Self as EntityView>::Entity: WithPrimaryKey<PrimaryKeyType = Self::PrimaryKeyType>,
{
    type PrimaryKeyType;
    type PrimaryKeyTags: TagList;

    fn primary_key(&self) -> &ExprBoxOfViewWithPrimaryKey<Self>;
}

pub trait AssociatedView<
    Parent: EntityWithView + WithPrimaryKey<PrimaryKeyType = Self::ForeignKeyType>,
    ForeignField: FieldMarkerWithView<
        Entity = <Self as EntityView>::Entity,
        FieldType = Self::ForeignKeyType,
        ViewTags = Self::ForeignKeyTags,
    >,
>: EntityView where
    <Self as EntityView>::Entity:
        Association<Parent, ForeignField, ForeignKeyType = Self::ForeignKeyType>,
{
    type ForeignKeyType: Value + Ord;
    type ForeignKeyTags: TagList;
    fn foreign_key(&self) -> &ExprBoxOfAssociatedView<Self, Parent, ForeignField>;
}

pub type TypeOfMarker<M> = <M as FieldMarker>::FieldType;
pub type TagOfMarker<M> = <M as FieldMarkerWithView>::ViewTags;

pub trait FieldMarkerWithView: FieldMarker
where
    <Self as FieldMarker>::Entity: EntityWithView,
    <Self as FieldMarker>::FieldType: Value,
{
    type ViewTags: TagList;

    fn columns() -> GenericArray<String, <Self::FieldType as Value>::L>
    where
        Self: Sized;

    fn view(
        entity_view: <Self::Entity as EntityWithView>::View,
    ) -> ExprViewBoxWithTag<Self::FieldType, Self::ViewTags>;
}

pub trait Identifiable: WithPrimaryKey + EntityWithView {
    fn get(id: Self::PrimaryKeyType) -> QueryResultFilter<Self>;
}

pub trait Deletable: Identifiable {
    fn delete(self) -> DeleteQueryResult<Self> {
        Self::get(self.primary_key().clone()).delete()
    }
}

pub trait Insertable<DB: Database, O>: for<'q> ArgSource<'q, DB, O> {
    type Entity: EntityWithView;
    type Source: for<'q> ArgSourceList<'q, DB, O>;

    fn insert(self) -> InsertQuery<DB, O, Self::Source>
    where
        Self: Sized;

    fn columns() -> Vec<String>
    where
        Self: Sized;
}
