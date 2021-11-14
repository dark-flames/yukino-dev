use crate::interface::def::FieldDefinition;
use crate::query::Alias;
use crate::view::{ExprView, Value};

pub trait FieldMarker {
    type Entity: Entity;

    fn field_name() -> &'static str;

    fn definition() -> &'static FieldDefinition;
}

pub trait Entity: Value {
    type View: EntityView<Entity = Self>;
}

pub trait EntityView: ExprView<Self::Entity> {
    type Entity: Entity;

    fn pure(alias: &Alias) -> Self
    where
        Self: Sized;
}
