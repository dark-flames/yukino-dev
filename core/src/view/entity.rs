use interface::YukinoEntity;
use query_builder::Alias;

use crate::view::{ExprView, Value};

pub trait EntityView: ExprView<Self::Entity> {
    type Entity: EntityWithView;

    fn pure(alias: &Alias) -> Self
    where
        Self: Sized;
}

pub trait EntityWithView: YukinoEntity + Value {
    type View: EntityView<Entity = Self>;
}
