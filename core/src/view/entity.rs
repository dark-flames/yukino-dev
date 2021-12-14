use generic_array::GenericArray;

use interface::{WithPrimaryKey, YukinoEntity};
use query_builder::Alias;

use crate::query::QueryResultFilter;
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

pub trait ViewWithPrimaryKey: EntityView
where <Self as EntityView>::Entity: WithPrimaryKey<Type=Self::Type> {
    type Type;
    type PrimaryKeyTag: TagList;

    fn primary_key(&self) -> &ExprViewBoxWithTag<Self::Type, Self::PrimaryKeyTag>;
}

pub trait FieldMarker {
    type Entity: EntityWithView;
    type FieldType: Value;
    type ViewTags: TagList;

    fn columns() -> GenericArray<String, <Self::FieldType as Value>::L> where Self: Sized;

    fn view(entity_view: <Self::Entity as EntityWithView>::View) -> ExprViewBoxWithTag<Self::FieldType, Self::ViewTags>;
}
