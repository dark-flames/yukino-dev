use generic_array::GenericArray;

use interface::{Association, FieldMarker, WithPrimaryKey, YukinoEntity};
use query_builder::{Alias, InsertQuery};

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

    fn all() -> QueryResultFilter<Self>;
}

pub type ExprBoxOfAssociatedView<V, P> = ExprViewBoxWithTag<
    <V as AssociatedView<P>>::ForeignKeyType,
    <V as AssociatedView<P>>::ForeignKeyTags,
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

pub trait AssociatedView<E: EntityWithView + WithPrimaryKey<PrimaryKeyType = Self::ForeignKeyType>>:
    EntityView
where
    <Self as EntityView>::Entity: Association<E, ForeignKeyType = Self::ForeignKeyType>,
{
    type ForeignKeyType;
    type ForeignKeyTags: TagList;
    fn foreign_key(&self) -> &ExprBoxOfAssociatedView<Self, E>;
}

pub trait FieldMarkerWithView: FieldMarker {
    type Entity: EntityWithView;
    type FieldType: Value;
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

pub trait Insertable: EntityWithView {
    fn insert(self) -> InsertQuery;
}
