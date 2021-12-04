use interface::YukinoEntity;
use query_builder::Alias;

use crate::query::QueryResultFilter;
use crate::view::{ExprView, Value, VerticalView};

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
